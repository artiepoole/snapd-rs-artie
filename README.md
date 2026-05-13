# test-team-please-ignore

## snapd-rs

Rust bindings for snapd. This allows applications in Rust to talk directly to snapd without writing their own API layer.

We use AI to generate the endpoint implementations and Rust types.

## setup
For megademo.ai. The agent has access to snapd source, so please use
```
git clone git@github.com:canonical/snapd.git
git clone git@github.com:ubuntu/app-center.git
```

to add snapd and appcenter as a subdir.


to update snapd to use 6 in order for workshop to work.
```
sudo snap refresh --channel=6/stable lxd
```
and install workshop
```
sudo snap install workshop --channel=latest/edge
```
then to initialise
```
workshop launch
```
and finally
```
workshop shell
> copilot
```
to enter the shell tool or alternatively
```
# Run copilot interactively
workshop run copilot
# Run copilot with a given prompt
workshop run copilot-prompt <prompt>
# E.g.
workshop run copilot-prompt how many times does the letter p occur in raspberry?
```
to go yolo mode.


# workshop usage

```
workshop launch
```

To see the list of all workshop quick actions, see `workshop.yaml`.
