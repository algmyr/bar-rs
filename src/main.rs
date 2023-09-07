use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::Duration;

use bar_rs::blocks;
use bar_rs::blocks::BlockInterface;
use bar_rs::events::InputEvent;

struct Block {
  block: Box<dyn BlockInterface + Send>,
  update_time: Duration,
}

impl Block {
  pub fn update(&mut self) { self.block.update().unwrap(); }

  pub fn to_string(&self) -> String { self.block.to_string() }

  #[allow(dead_code)]
  pub fn handle_input(&mut self, event: &InputEvent) {
    if let Ok(true) = self.block.handle_input(event) {
      self.update();
    }
  }

  pub fn name(&self) -> &str { self.block.name() }
}

fn setup_printer(blocks: Vec<Arc<Mutex<Block>>>) -> JoinHandle<()> {
  std::thread::spawn(move || {
    println!(r#"{{ "version": 1, "click_events": true  }}"#);
    println!("[");
    loop {
      println!(
        "[{}],",
        blocks
          .iter()
          .map(|block| block.lock().unwrap().to_string())
          .collect::<Vec<String>>()
          .join(",")
      );
      std::io::stdout().flush().unwrap();
      std::thread::sleep(std::time::Duration::from_millis(250));
    }
  })
}

fn setup_block_updates(blocks: Vec<Arc<Mutex<Block>>>) -> Vec<JoinHandle<()>> {
  blocks
    .into_iter()
    .map(|block| {
      std::thread::spawn(move || loop {
        block.lock().unwrap().update();
        let dur = block.lock().unwrap().update_time;
        std::thread::sleep(dur);
      })
    })
    .collect()
}

fn setup_input_handler(blocks: Vec<Arc<Mutex<Block>>>) -> JoinHandle<()> {
  std::thread::spawn(move || {
    let mut lines = std::io::stdin().lines();
    lines.next().unwrap().unwrap();
  
    while let Some(Ok(line)) = lines.next() {
      let line = line.trim_start_matches(',');
      let event: InputEvent = serde_json::from_str(line).unwrap();

      for block in blocks.iter() {
        let mut block = block.lock().unwrap();
        if event.name.starts_with(block.name()) {
          block.handle_input(&event);
        }
      }
    }
  })
}

macro_rules! setup_block {
  ($block:ty, $dur:expr) => {
    Arc::new(Mutex::new(Block {
      block: Box::new(<$block>::default()),
      update_time: $dur,
    }))
  };
}

#[allow(dead_code)]
const NEVER: Duration = Duration::from_secs(1000000000);
#[allow(dead_code)]
const SUB_MINUTE: Duration = Duration::from_secs(30);
#[allow(dead_code)]
const SECOND: Duration = Duration::from_secs(1);
#[allow(dead_code)]
const SUB_SECOND: Duration = Duration::from_millis(250);

fn main() {
  let args: Vec<String> = std::env::args().collect();

  if args.len() < 2 {
    println!("Usage: {} <output name>", args[0]);
    std::process::exit(1);
  }

  let blocks: Vec<Arc<Mutex<Block>>> = if args[1] == "DP-2" {
    vec![
    setup_block!(blocks::MediaBlock, SUB_SECOND),
    setup_block!(blocks::Separator, NEVER),
    setup_block!(blocks::LoadBlock, SECOND),
    setup_block!(blocks::Separator, NEVER),
    setup_block!(blocks::VolumeBlock, SUB_SECOND),
    setup_block!(blocks::Separator, NEVER),
    setup_block!(blocks::NetworkBlock, SECOND),
    setup_block!(blocks::Separator, NEVER),
    setup_block!(blocks::DateBlock, SUB_MINUTE),
    setup_block!(blocks::Separator, NEVER),
    setup_block!(blocks::ClockBlock, SUB_MINUTE),
  ]
  } else {
    vec![
    setup_block!(blocks::DateBlock, SUB_MINUTE),
    setup_block!(blocks::Separator, NEVER),
    setup_block!(blocks::ClockBlock, SUB_MINUTE),
  ]

  };

  let block_threads = setup_block_updates(blocks.clone());
  let input_thread = setup_input_handler(blocks.clone());
  let printer_thread = setup_printer(blocks.clone());

  // Rejoin threads.
  printer_thread.join().unwrap();
  input_thread.join().unwrap();
  block_threads.into_iter().for_each(|t| t.join().unwrap());
}
