#include "tmux.h"
#include "rust-glue.h"

int glue_window_panes_foreach_safe(struct window_panes *panes, void *context, int (*f)(struct window_pane *, void *context)) {
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
