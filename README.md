# What is it?

This program periodically scans all the child cgroups of the specified parent cgroup and analyzes
memory consumption using control file `memory.stat` of the cgroup memory resource controller.
When cgroup cache usage is higher than the specified threshold, it triggers a forced page reclaim
via control file `memory.force_empty`, but not more often than once in the specified time frame.

# Why do you need it?

Linux cgroup-aware out-of-memory (OOM) killer accounts RSS, kmem, and cache when calculating memory usage for a cgroup.
A process that is running in a cgroup cannot directly control its cache usage.

It is a good practice in Kubernetes to set a memory limit for containers.
However, even if your program does not consume more than the limit, OOM killer can kill your
container if the total usage (RSS+cache) is bigger than the limit.

# Usage

`kube-node-memory-flusher [OPTIONS]`

Options:
- `--parent` path to the parent cgroup
- `--threshold` cache usage threshold in %, bytes, or other units
- `--interval` how frequently to check cache usage for all cgroups in seconds
- `--cooldown` the minimum time to wait in seconds between forcing page reclaim

Set environment variable `RUST_LOG=info` to see what cgroups are detected and reclaimed.

# Running as a process on host

Assuming you want to run `kube-node-memory-flusher` on a Kubernetes node, you can use the following
systemd unit file:


```
[Unit]
Description=kube-node-memory-flusher
After=network.target

[Service]
Type=simple
ExecStart=/usr/local/bin/kube-node-memory-flusher \
    --parent /sys/fs/cgroup/memory/kubepods \
    --threshold 25%
Restart=always

[Install]
WantedBy=multi-user.target
```

# Running as DaemonSet in Kubernetes

Example of DaemonSet manifest for kube-node-memory-flusher that mounts `/sys/fs/cgroup` from the host.

# Details

In Kubernetes, cgroups for container in Pods have complex hierarchy that includes Pod QoS class, for
example:

```
/sys/fs/cgroup/memory/kubepods/
├── podA
│   └── containerA1
├── burstable
│   └── podB
│       └── containerB1
└── besteffort
    └── podC
        └── containerC1
```

While it is possible to monitor memory consumption for the parent cgroups that correspond to Pods or
QoS classes, `kube-node-memory-flusher` does this only for cgroups that correspond to containers.
For Kubernetes, set `--parent` to `/sys/fs/cgroup/memory/kubepods`.

Only the cgroup memory resource controller v1 is supported, see
https://www.kernel.org/doc/Documentation/cgroup-v1/memory.txt.
