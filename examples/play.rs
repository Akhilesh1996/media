extern crate servo_media;

use servo_media::audio::gain_node::{GainNodeMessage, GainNodeOptions};
use servo_media::audio::node::{AudioNodeMessage, AudioNodeType};
use servo_media::audio::oscillator_node::OscillatorNodeMessage;
use servo_media::audio::param::{RampKind, UserAutomationEvent};
use servo_media::ServoMedia;
use std::{thread, time};

fn main() {
    if let Ok(servo_media) = ServoMedia::get() {
        let mut graph = servo_media.create_audio_graph();
        graph.create_node(AudioNodeType::OscillatorNode(Default::default()));
        let mut options = GainNodeOptions::default();
        options.gain = 0.5;
        graph.create_node(AudioNodeType::GainNode(options));
        graph.message_node(
            0,
            AudioNodeMessage::OscillatorNode(OscillatorNodeMessage::Start(0.)),
        );
        graph.message_node(
            0,
            AudioNodeMessage::OscillatorNode(OscillatorNodeMessage::Stop(3.)),
        );
        assert_eq!(graph.current_time(), 0.);
        graph.resume();
        // change frequency at 0.5s and 1s, then ramp up linearly till 1.7s, then ramp down till 2.5s
        // change gain at 0.75s, then ramp to full gain reached at 1.5s
        graph.message_node(
            0,
            AudioNodeMessage::OscillatorNode(OscillatorNodeMessage::SetFrequency(
                UserAutomationEvent::SetValueAtTime(110., 0.5),
            )),
        );
        graph.message_node(
            0,
            AudioNodeMessage::OscillatorNode(OscillatorNodeMessage::SetFrequency(
                UserAutomationEvent::SetValueAtTime(220., 1.),
            )),
        );
        graph.message_node(
            1,
            AudioNodeMessage::GainNode(GainNodeMessage::SetGain(
                UserAutomationEvent::SetValueAtTime(0.25, 0.75),
            )),
        );
        thread::sleep(time::Duration::from_millis(1200));
        graph.suspend();
        thread::sleep(time::Duration::from_millis(500));
        graph.resume();
        let current_time = graph.current_time();
        assert!(current_time > 0.);
        // Leave some time to enjoy the silence after stopping the
        // oscillator node.
        thread::sleep(time::Duration::from_millis(5000));
        // And check that we keep incrementing playback time.
        assert!(current_time < graph.current_time());
        graph.close();
    } else {
        unreachable!();
    }
}
