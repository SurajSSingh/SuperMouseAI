import { Dialog as DialogPrimitive } from "bits-ui";

import Title from "./dialog-title.svelte";
import Overlay from "./dialog-overlay.svelte";
import Content from "./dialog-content.svelte";
import Description from "./dialog-description.svelte";

const Root = DialogPrimitive.Root;
const Trigger = DialogPrimitive.Trigger;
const Close = DialogPrimitive.Close;
const Portal = DialogPrimitive.Portal;

export {
    Root,
    Title,
    Portal,
    Trigger,
    Overlay,
    Content,
    Description,
    Close,
    //
    Root as Dialog,
    Title as DialogTitle,
    Portal as DialogPortal,
    Trigger as DialogTrigger,
    Overlay as DialogOverlay,
    Content as DialogContent,
    Description as DialogDescription,
    Close as DialogClose,
};