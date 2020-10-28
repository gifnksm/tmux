#include "tmux.h"
#include "rust-glue.h"

int glue_window_panes_foreach_safe(struct window_panes *panes, void *context, int (*f)(struct window_pane *, void *)) {
    struct window_pane *loopwp, *tmpwp;
    int ret = 0;
    TAILQ_FOREACH_SAFE(loopwp, panes, entry, tmpwp) {
        ret = f(loopwp, context);
        if (ret) {
            break;
        }
    }
    return ret;
}

int glue_winlinks_foreach_safe(struct winlinks *wls, void *context, int (*f)(struct winlink *, void *)) {
    struct winlink *loopwl, *tmpwl;
    int ret = 0;
    RB_FOREACH_SAFE(loopwl, winlinks, wls, tmpwl) {
        ret = f(loopwl, context);
        if (ret) {
            break;
        }
    }
    return ret;
}

int glue_sessions_foreach_safe(struct sessions *ss, void *context, int (*f)(struct session *, void *)) {
    struct session *loops, *tmps;
    int ret = 0;
    RB_FOREACH_SAFE(loops, sessions, ss, tmps) {
        ret = f(loops, context);
        if (ret) {
            break;
        }
    }
    return ret;
}
