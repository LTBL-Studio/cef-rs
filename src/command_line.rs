use cef_sys::{cef_command_line_create, cef_command_line_t};

use crate::{
    string::{CefString, CefStringError, CefStringList, CefStringMap},
    wrapper,
};

wrapper!(
    #[doc = "See [cef_command_line_t] for more documentation."]
    #[derive(Debug, Clone)]
    pub struct CommandLine(cef_command_line_t);
    pub fn reset(&self);
);

// #[doc = "\n Initialize the command line with the specified |argc| and |argv| values.\n The first argument must be the name of the program. This function is only\n supported on non-Windows platforms.\n"]
// pub init_from_argv: ::std::option::Option<
//     unsafe extern "C" fn(
//         self_: *mut _cef_command_line_t,
//         argc: ::std::os::raw::c_int,
//         argv: *const *const ::std::os::raw::c_char,
//     ),
// >,

// #[doc = "\n Retrieve the original command line string as a vector of strings. The argv\n array: `{ program, [(--|-|/)switch[=value]]*, [--], [argument]* }`\n"]
// pub get_argv: ::std::option::Option<
//     unsafe extern "C" fn(self_: *mut _cef_command_line_t, argv: cef_string_list_t),
// >,

impl CommandLine {
    pub fn is_valid(&self) -> bool {
        let is_valid = self.0.is_valid.unwrap();

        unsafe { is_valid(self.0.get_raw()) > 0 }
    }

    pub fn is_read_only(&self) -> bool {
        let is_read_only = self.0.is_read_only.unwrap();

        unsafe { is_read_only(self.0.get_raw()) > 0 }
    }

    pub fn copy(&self) -> Self {
        let copy = self.0.copy.unwrap();

        unsafe { Self::from_raw(copy(self.0.get_raw())) }
    }

    /// Retrieves the original command line string as a vector of strings. The argv\n array: `{ program, [(--|-|/)switch[=value]]*, [--], [argument]* }`
    pub fn get_argv(&self) -> CefStringList {
        let get_argv = self.0.get_argv.unwrap();
        let mut argv = CefStringList::default();

        unsafe { get_argv(self.0.get_raw(), argv.get_raw_mut()) }

        argv
    }

    pub fn get_command_line_string(&self) -> Result<CefString, CefStringError> {
        let get_command_line_string = self.0.get_command_line_string.unwrap();

        unsafe { CefString::from_userfree_cef(get_command_line_string(self.0.get_raw())) }
    }

    pub fn get_program(&self) -> CefString {
        let get_program = self.0.get_program.unwrap();

        unsafe {
            CefString::from_userfree_cef(get_program(self.0.get_raw()))
                .expect("Error getting program name")
        }
    }

    pub fn set_program(&mut self, program: CefString) {
        let set_program = self.0.set_program.unwrap();

        unsafe { set_program(self.0.get_raw(), &program.get_raw()) };
    }

    pub fn has_switches(&self) -> bool {
        let has_switches = self.0.has_switches.unwrap();

        unsafe { has_switches(self.0.get_raw()) > 0 }
    }

    /// Returns the map of switch names and values
    pub fn get_switches(&self) -> CefStringMap {
        let get_switches = self.0.get_switches.unwrap();
        let mut map = CefStringMap::default();

        unsafe { get_switches(self.0.get_raw(), map.get_raw_mut()) };

        map
    }

    pub fn has_switch(&self, name: CefString) -> bool {
        let has_switch = self.0.has_switch.unwrap();

        unsafe { has_switch(self.0.get_raw(), &name.get_raw()) > 0 }
    }

    pub fn get_switch_value(&self, name: CefString) -> CefString {
        let get_switch_value = self.0.get_switch_value.unwrap();

        unsafe {
            CefString::from_userfree_cef(get_switch_value(self.0.get_raw(), &name.get_raw()))
                .expect("Error getting program name")
        }
    }

    pub fn append_switch(&mut self, name: CefString) {
        let append_switch = self.0.append_switch.unwrap();

        unsafe { append_switch(self.0.get_raw(), &name.get_raw()) };
    }

    pub fn append_switch_with_value(&mut self, name: CefString, value: CefString) {
        let append_switch_with_value = self.0.append_switch_with_value.unwrap();

        unsafe { append_switch_with_value(self.0.get_raw(), &name.get_raw(), &value.get_raw()) };
    }

    pub fn has_arguments(&self) -> bool {
        let has_arguments = self.0.has_arguments.unwrap();

        unsafe { has_arguments(self.0.get_raw()) > 0 }
    }

    /// Gets the remaining command line arguments
    pub fn get_arguments(&self) -> CefStringList {
        let get_arguments = self.0.get_arguments.unwrap();
        let mut arguments = CefStringList::default();

        unsafe { get_arguments(self.0.get_raw(), arguments.get_raw_mut()) };

        arguments
    }

    /// Adds an argument to the end of the command line
    pub fn append_argument(&self, argument: CefString) {
        let append_argument = self.0.append_argument.unwrap();

        unsafe { append_argument(self.0.get_raw(), &argument.get_raw()) };
    }
    
    /// Inserts a command before the current command. Common for debuggers, like `valgrind` or `gdb --args\`
    pub fn prepend_wrapper(&self, wrapper: CefString) {
        let prepend_wrapper = self.0.prepend_wrapper.unwrap();

        unsafe { prepend_wrapper(self.0.get_raw(), &wrapper.get_raw()) };
    }
}

impl Default for CommandLine {
    fn default() -> Self {
        unsafe { Self::from_raw(cef_command_line_create()) }
    }
}

#[cfg(target_os = "windows")]
impl From<&str> for CommandLine {
    fn from(value: &str) -> Self {
        let cmd_line = Self::default();
        let cef_string = CefString::from(value).get_raw();
        unsafe { cmd_line.0.init_from_string.unwrap()(cmd_line.0.get_raw(), &cef_string) };
        cmd_line
    }
}
