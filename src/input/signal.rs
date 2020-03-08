use crate::input::InputFlag;

#[derive(Copy, Clone, Default)]
pub struct InputSignal {
    pub(crate) is_down: InputFlag,
    pub(crate) is_push: InputFlag,
    pub(crate) is_release: InputFlag,
}

impl InputSignal {
    pub fn is_down_flag(&self) -> InputFlag {
        self.is_down
    }
    pub fn is_down(&self, flag: InputFlag) -> bool {
        self.is_down.contains(flag)
    }
    pub fn is_push_flag(&self) -> InputFlag {
        self.is_push
    }
    pub fn is_push(&self, flag: InputFlag) -> bool {
        self.is_push.contains(flag)
    }
    pub fn is_release_flag(&self) -> InputFlag {
        self.is_release
    }
    pub fn is_release(&self, flag: InputFlag) -> bool {
        self.is_release.contains(flag)
    }
}
