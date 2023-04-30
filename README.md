# Channel CI

Todo list. This to-do list is not complete and may change as different tasks are accomplished.

- [x] Create Jobs
    - [ ] Manual Triggers
    - [ ] Automated Triggers (Webhooks, other git related things, etc.)
    - [ ] Timed builds
        - Maybe even a setting to wait X time after a commit to the repository / trigger before actually running a job
- [ ] Run Jobs (see more)
    - Needs more abstraction around creating and sending to allow multiple triggers (i.e. Github Webhooks, Git, Manually triggering, etc.)
    - [x] Docker
    - [ ] LXC
    - [ ] Other machines, vms, etc.
- [ ] Get Jobs
    - [x] Specific jobs
    - [x] All jobs
    - [ ] Filtered search/search in general
- [ ] Create runners
    - [ ] More detailed creation options and more runner options overall
- [ ] Get Runners (Same as jobs, please. Search, all, specific.).
- [ ] Improve errors around every aspect of ChannelCi