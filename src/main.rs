use anyhow::Error;
use gst::prelude::*;

mod signaller;

fn main() -> Result<(), Error> {
    gst::init()?;

    let custom_signaller = signaller::Signaller::new();
    let webrtcsink = webrtcsink::webrtcsink::WebRTCSink::with_signaller(Box::new(custom_signaller));

    let pipeline = gst::Pipeline::new(None);

    // let vsrc =        gst::ElementFactory::make_with_properties("videotestsrc", &[("", &"".to_value())]).unwrap();
    let vsrc = gst::parse_launch("videotestsrc pattern=ball")?;

    pipeline
        .add_many(&[&vsrc, webrtcsink.upcast_ref()])
        .unwrap();
    vsrc.link(webrtcsink.upcast_ref::<gst::Element>()).unwrap();

    let bus = pipeline.bus().unwrap();

    pipeline.set_state(gst::State::Playing).unwrap();

    let _msg = bus.timed_pop_filtered(gst::ClockTime::NONE, &[gst::MessageType::Eos]);

    pipeline.set_state(gst::State::Null).unwrap();

    Ok(())
}
