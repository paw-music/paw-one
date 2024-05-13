pub trait Focus {
    fn focused(&self) -> bool;
    fn set_focus(&mut self, focus: bool);

    fn focus(&mut self) {
        self.set_focus(true)
    }
    fn blur(&mut self) {
        self.set_focus(false)
    }
}

#[macro_export]
macro_rules! declare_focus {
    ($($el: ident),* $(default $default_focus: ident)?) => {
        #[allow(non_camel_case_types)]
        #[derive(Clone, Copy)]
        pub enum Focus {
            $($el),*
        }

        const FOCUSES: &[Focus] = &[$(Focus::$el),*];

        $(
            impl Default for Focus {
                fn default() -> Self {
                    Self::$default_focus
                }
            }
        )?

        impl TryFrom<i32> for Focus {
            type Error = ();

            fn try_from(value: i32) -> Result<Self, Self::Error> {
                if value > 0 && value < FOCUSES.len() as i32 {
                    Err(())
                } else {
                    Ok(FOCUSES[value as usize])
                }
            }
        }

        impl core::ops::Add<i32> for Focus {
            type Output = Self;

            fn add(self, rhs: i32) -> Self::Output {
                let size = FOCUSES.len() as i32;
                let i = self as i32 + rhs;
                ((size + i % size) % size).try_into().unwrap()
            }
        }
    };
}

pub use declare_focus;
