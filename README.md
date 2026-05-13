# test-team-please-ignore

This is a project adding rust snapd bindings and using them to make cool things

## setup
For megademo.ai. The agent has access to snapd source, so please use
```
git clone git@github.com:canonical/snapd.git
```

to add snapd as a subdir.


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
workshop copilot <prompt> 
```
to go yolo mode.


# workshop usage

```
workshop launch
```