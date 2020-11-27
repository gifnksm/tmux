pub(crate) type Msgtype = libc::c_uint;

/// Message codes.
pub(crate) mod code {
    use super::Msgtype;

    pub(crate) const VERSION: Msgtype = 12;

    pub(crate) const IDENTIFY_FLAGS: Msgtype = 100;
    pub(crate) const IDENTIFY_TERM: Msgtype = 101;
    pub(crate) const IDENTIFY_TTYNAME: Msgtype = 102;
    pub(crate) const IDENTIFY_OLDCWD: Msgtype = 103;
    pub(crate) const IDENTIFY_STDIN: Msgtype = 104;
    pub(crate) const IDENTIFY_ENVIRON: Msgtype = 105;
    pub(crate) const IDENTIFY_DONE: Msgtype = 106;
    pub(crate) const IDENTIFY_CLIENTPID: Msgtype = 107;
    pub(crate) const IDENTIFY_CWD: Msgtype = 108;
    pub(crate) const IDENTIFY_FEATURES: Msgtype = 109;
    pub(crate) const IDENTIFY_STDOUT: Msgtype = 110;
    pub(crate) const IDENTIFY_LONGFLAGS: Msgtype = 111;

    pub(crate) const COMMAND: Msgtype = 200;
    pub(crate) const DETACH: Msgtype = 201;
    pub(crate) const DETACHKILL: Msgtype = 202;
    pub(crate) const EXIT: Msgtype = 203;
    pub(crate) const EXITED: Msgtype = 204;
    pub(crate) const EXITING: Msgtype = 205;
    pub(crate) const LOCK: Msgtype = 206;
    pub(crate) const READY: Msgtype = 207;
    pub(crate) const RESIZE: Msgtype = 208;
    pub(crate) const SHELL: Msgtype = 209;
    pub(crate) const SHUTDOWN: Msgtype = 210;
    pub(crate) const OLDSTDERR: Msgtype = 211;
    pub(crate) const OLDSTDIN: Msgtype = 212;
    pub(crate) const OLDSTDOUT: Msgtype = 213;
    pub(crate) const SUSPEND: Msgtype = 214;
    pub(crate) const UNLOCK: Msgtype = 215;
    pub(crate) const WAKEUP: Msgtype = 216;
    pub(crate) const EXEC: Msgtype = 217;
    pub(crate) const FLAGS: Msgtype = 218;

    pub(crate) const READ_OPEN: Msgtype = 300;
    pub(crate) const READ: Msgtype = 301;
    pub(crate) const READ_DONE: Msgtype = 302;
    pub(crate) const WRITE_OPEN: Msgtype = 303;
    pub(crate) const WRITE: Msgtype = 304;
    pub(crate) const WRITE_READY: Msgtype = 305;
    pub(crate) const WRITE_CLOSE: Msgtype = 306;
}
