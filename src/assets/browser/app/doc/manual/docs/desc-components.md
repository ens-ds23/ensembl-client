# Major Components

## Mainloop and Scheduling

As much as possible runs out of a mainloop driven by requestanimationframe
events (hereafter "raf events"). A dedicated scheduler attempts to ensure
that the raf hanlder returns in plenty of time.

### Jank Detection

Visual smoothness benefits from regular updates of visual content even if
that is less than the maximum available rate. Variable update rates tend
to be noticable even if correctly timed. To this end draw events are
scheduled at 20Hz, 30Hz, or 60Hz and no intermediate speed. Raf events
which correspond to this (parhaps reduced rate) are called on-beats. In
an on-beat drawing gets the top scheduler priority. The other events,
off-beats, do not include the drawing callbacks. 

A jank detector updates the optimal rate for the hardware based on a 
record of performance of earlier events. Each raf is tested to see if
it has *burst*, that is met or exceeded its allocated time. This is a
simple boolean. The record and timing of far bursts is the input to the
jank detector. The output is the rate of on-requests, known as the "gear"
or "timesig".

This behaviour and fix is the result of observed behaviour, not 
conjecture, and follows various ineffective "simpler" fixes.

### Jank Detection Algorithm

It has not proved possible to reliably tell which gear minimises raf
bursts except by applying gears and seeing what happens. The jank-detection
algorithm takes the raf burst flag and the current time and sets the gear.

As experimentation is required and it amounts to a "bang-bang" controller
(ie non-proportional) the best behaviour we can expect to observe is 
slow "hunting" between an under- and over-performant state (ie a slow
oscillation).

The opposite behaviour to "hunting" here is "moving" which is a conserted
movement through gears in one direction or another. The jank-detection
algorithm considers to successive changes in the same direction to be moving,
two in opposite directions to be hunting.

If the period between changes during "hunting" is too long we will never
take advantage of performance changes (such as recovery after one-off
events). If too short, the rate becomes unstable and jank is observed.

If a burst occurs, the rate is decreased immediately. Following this event,
a grace period ensures that no opposing rate increase occurs for a fixed
(wall-clock) time to avoid excessively-quick hunting on a stable, quiescent
system. Once the grace period expires, the rate is increased again
to allow potential recovery. The grace period increases exponentially 
(fibbionacci, to a fixed limit) all the while the rate is hunting, as long as
a hunt occured during the grace period (indicating that the higher rate is
not stable). It decreases by the same exponential curve if the hunting period
is longer than expected (indicating a system stable at the faster rate). This
ensures that the rate "settles" on a stable system. When moving is detected 
the rate is reset to allow fast accommodation of changes.

### Scheduling algorithm

The scheduler accepts callbacks which are placed in an on-beat
or off-beat queues, as requested by the caller, and of a certain priority.
This priority should be a small, positive integer. The list of on-beat
queues and the list of off-beat queues (each a *queuelist*) are combined
to form the scheduler.

For an on-beat, the on queuelist is run, followed by the off queuelist.
For an off-beat only the off queuelist is run.

An off-beat raf is considered to have burst if the off-beat queuelist
bursts as it is the only queuelist run in an off-beat.

An on-beat raf is only considered to have burst if the on-beat queuelist
bursts as this is all that's "expected" to run on an on-beat. The exception
is when running in gear one, ie 60Hz. In this case either queuelist
bursting counts as a burst as there are no other opportunities to run
off-beat tasks.

Within a queuelist are queues. Queues are arranged in order of priority.
Each queue is run from lowest priority to highest in round-robin. Note
that low-priority queues never get run if high-priorirty queues consume
all the time, so be cautious with fine-graining priorities. At present
queues are stored in a vec so please keep priorities ultra-compact.

### Code Structure

* `controller/scheduler` -- the scheduler
    * `schedgroup.rs` -- a group of tasks (which are dropped when the group is)
    * `schedmain.rs` -- main implementation of scheduler "tick"
    * `schedqueue.rs` -- a queue type in the algorithm above
    * `schedqueuelist.rs` -- the queuelist type in the algorithm above
    * `schedrun.rs` -- an object passed to the callback to help control the scheduler
    * `schedtask.rs` -- the internal datatype used to represent the callback
    * `scheduler.rs` -- main entry point to the scheduler

### Instrumentation

Each queue run and task are integrated with the blackbox via the `scheduler-*`
event stream which capture stream performance. The `scheduler` and
`scheduler-jank` streams provide further logging.

### Tasks

The following tasks are currently implemented:

* `scheduler-task-http-manager` -- issuing http requests
* `scheduler-task-xfer` -- http data repsonses
* `scheduler-task-tácode` -- tánaiste interpreter
* `scheduler-task-blackbox` -- blackbox callback
* `scheduler-task-resizer` -- canvas resize detection
* `scheduler-task-report` -- state-reporting javascript events
* `scheduler-task-viewport-report` -- position-reporting javascript events
* `scheduler-task-physics` -- mouse movement (left/right)
* `scheduler-task-optical` -- mouse movement (in/out)
* `scheduler-task-compositor` -- converting shapes into WebGL primitives
* `scheduler-task-draw` -- drawing (on-beat)

Except for `scheduler-task-draw` all others are off-beat tasks.


## Instrumentation

Instrumentation code exists to assist development and debugging of the browser
app. It is generally not available in deployed builds to reduce file size and
improve performance.

### Black Box

The black box system periodically sends logs and data sets to the backend
server. There, the server stores this data in filesi, according to server
configuration. The backend server also controls which datasets and logs are
captured, and the frequency of callbacks. These are controlled by a
configuration file. This file is sent to the client as the response to each
request containing data.

The primary use of the black box is to monitor performance.

Data sets are arranged into streams which are represented by a string. Each
log or data capture includes the stream to which it belongs. The server
configures which logs and data to capture and their destination file based on
the originating stream.

A stack allows a log message to be contextualised by pusing and popping string
context as to its location. This allows the same log messages to be divided
into subsets. The stack mechanism is relatively heavyweight and designed only
for debugging rather than long-lived instrumentation.

The blackbox supports abstract drivers. These use different mechanisms to
report to the server. Currently only an http driver and null driver are
implemented.

#### Important Files

* `app/data/blackbox` -- contains rust files implementing the blackbox:
    * core
        * `blackbox.rs` -- high-level static API for use in macros.
        * `blackboxstate.rs` -- current state of blackbox on client.
        * `bbreportstream.rs` -- implements a single stream's pending contents.
    * drivers
        * `blackboxdriver.rs` -- facade around driver implementations.
        * `httpblackboxdriver.rs` -- driver implementation for HTTP callbacks.
        * `nullblackboxdriver.rs` -- no-op driver implementation.
        * `stubdriver.rs` -- non-implementation of driver for production builds
* `macros.rs` -- macros to use blackbox in code.
* `debug_mode.yaml` -- server side configuration
* `POST /browser/debug` -- API endpoint

#### Macros

The blackbox is used exclusively through macros.

* `bb_time(stream,block)` -- execute block, timing it and adding to stream dataset.
* `bb_metronome(stream)` -- add to dataset interval between each call to this macro for this stream
* `bb_log(stream,format,args)` -- write formatted log message to stream
* `bb_stack(string,block)` -- push "string" onto stack and execute block

#### Payload Format (POST /browser/debug)
##### Client to Server Payload

POST request with raw JSON payload.

```
{
  "instance_id": "<string>", /* browser-identifying string */
  "streams": {
    "<stream-name>": {
      "reports": [
        {
          "time": <number>,   /* ms since unix epoch */
          "text": "<string>", /* log contents */
          "stack": "<string>" /* stack at time of logging  */
        },...
      ],
      "dataset": [<number>,...] /* dataset. Key may be absent if not configured */
      }
    }, ...
  }
}
```

##### Server to Client

POST request response with raw JSON payload. The whole contents of
`debug_mode.yaml` are sent (ranscoded), though the server only reacts to some
keys.

```
{
  "enabled": ["<string>",...], /* streams to enable */
  "dataset": ["<string>",...], /* datasets to enable (stream must also be enabled) */
  "interval": <number>, /* requested interval (in seconds) between updates */
}
```

