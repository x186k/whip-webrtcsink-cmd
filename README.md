# WHIP based webrtcsink custom signaller example

This repo generates an executable which instantiates a gStreamer pipeline which sends to
a WHIP endpoint using the `webrtcsink` plugin.

Forked from: https://github.com/centricular/webrtcsink-custom-signaller


This is a simple example to demonstrate how to use webrtcsink with a custom
signaller implementation.

Run with:

```shell
WHIPURL=http://192.168.86.3:7080/whip/endpoint/foo cargo run
```

If you have a valid WHIP endpoint at WHIPURL, you that endpoint will receive a video stream of the
`videotestsrc` bouncing ball test pattern.

Some notes:

* This repo does NOT generate a gStreamer plugin that allows WHIP to be used with `webrtcsink`, but an executable.
* This has been tested with Janus and Janus' simple-whip-server and the videoroom plugin, and does work in this scenarion.
* This uses a timer to collect ICE candidates, form a full SDP, and does not use the WHIP PATCH HTTP request to send candidates one at a time.
* This is an early nascent implementation of WHIP, and probably could use some tweaks for production environments.
* The HTTP DELETE request under WHIP may need some work.


You can see more output when you run like:

```shell
GST_DEBUG=webrtcsink:4 RUST_BACKTRACE=1 WHIPURL=http://192.168.86.3:7080/whip/endpoint/foo cargo run
```
