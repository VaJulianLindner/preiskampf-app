export function transition(domUpdateFunction, callbacks) {
    // TODO maybe introduce a flag, that will be preventing multiple transitions from starting simultaneousy
    if (!document.startViewTransition) {
        domUpdateFunction();
        return;
    }

    callbacks?.init?.();
    const transitionHandle = startTransition({ update: domUpdateFunction, types: [] });

    if (!transitionHandle) {
        console.error("couldnt start transition");
        return;
    }

    onReady(transitionHandle, callbacks?.ready);
    onFinished(transitionHandle, callbacks?.finished);
    return onUpdateCallbackDone(transitionHandle, callbacks?.updateCallbackDone);
}

function startTransition(cfg) {
    try {
        return document.startViewTransition(cfg);
    } catch (e) {
        if (e.toString().indexOf("Overload resolution failed") !== -1) {
            return document.startViewTransition(cfg.update);
        }
    }
}

async function onReady(transitionHandle, cb) {
    try {
        await transitionHandle.ready;
        cb?.();
    } catch (e) {
        console.error("transition failed:", e);
    }
}

async function onFinished(transitionHandle, cb) {
    await transitionHandle.updateCallbackDone;
    cb?.();
}

async function onUpdateCallbackDone(transitionHandle, cb) {
    const before = new Date().getTime();
    await transitionHandle.finished;
    console.debug("onUpdateCallbackDone took", (new Date().getTime() - before));
    cb?.();
}