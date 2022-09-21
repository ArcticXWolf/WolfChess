use chess::Color;
use std::time::Duration;
use tokio::{
    sync::watch::{self, Receiver, Sender},
    time::{sleep_until, Instant},
};
use vampirc_uci::UciTimeControl;

pub struct TimeBroker {
    cancel_sender: Sender<bool>,
    cancel_receiver: Receiver<bool>,
    duration: Option<Duration>,
}

impl TimeBroker {
    pub fn new() -> TimeBroker {
        let (cancel_sender, cancel_receiver) = watch::channel(false);
        TimeBroker {
            cancel_sender: cancel_sender,
            cancel_receiver: cancel_receiver,
            duration: None,
        }
    }

    pub fn seed_time_control(&mut self, own_color: Color, time_control: &UciTimeControl) {
        self.duration = match time_control {
            UciTimeControl::MoveTime(duration) => duration.to_std().map_or(None, |d| Some(d)),
            UciTimeControl::TimeLeft {
                white_increment,
                black_increment,
                ..
            } => match own_color {
                Color::White => white_increment.map_or(Some(Duration::from_secs(10)), |d| {
                    d.to_std()
                        .map_or(Some(Duration::from_secs(10)), |ds| Some(ds))
                }),
                Color::Black => black_increment.map_or(Some(Duration::from_secs(10)), |d| {
                    d.to_std()
                        .map_or(Some(Duration::from_secs(10)), |ds| Some(ds))
                }),
            },
            _ => None,
        };
        if self.duration == Some(Duration::ZERO) {
            self.duration = Some(Duration::from_secs(10));
        }
        println!("info string Duration set: {:?}", self.duration);
    }

    pub fn start_timer(&mut self) -> Option<Receiver<bool>> {
        if self.duration.is_none() {
            return None;
        }
        let (cancel_sender, cancel_receiver) = watch::channel(false);

        let moved_duration = self.duration.unwrap();

        tokio::spawn(async move {
            println!("info string TimerTask started");
            sleep_until(Instant::now() + moved_duration).await;
            cancel_sender.send(true).unwrap();
            println!("info string TimerTask shutdown");
        });
        Some(cancel_receiver)
    }

    pub fn send_stop(&self) {
        self.cancel_sender.send(true).unwrap();
    }

    pub fn get_cancel_receiver(&self) -> Receiver<bool> {
        return self.cancel_receiver.clone();
    }
}
