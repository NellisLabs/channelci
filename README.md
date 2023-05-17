# Channel CI

Todo list. This to-do list is not complete and may change as different tasks are accomplished.

- [ ] Move over from the old common repo to the new one in this repo / folder / workspace / whatever **IMPORTANT**
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
- [x] Improve errors around every aspect of ChannelCi
    - This should mostly be done so ima check it off. No major part in the entire system should've been left out cuz it would've errored, hopefully.

### Just some stuff for devs or people working on it to remember:
- Object Types:
    - All types *should* have a respective db table to go along with them
    - 0: Project
    - 1: Pipeline

- Trigger "service" column:
    - 0: Github