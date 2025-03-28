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
  Close,
  Close as DialogClose,
  Content,
  Content as DialogContent,
  Description,
  Description as DialogDescription,
  Overlay,
  Overlay as DialogOverlay,
  Portal,
  Portal as DialogPortal,
  Root,
  //
  Root as Dialog,
  Title,
  Title as DialogTitle,
  Trigger,
  Trigger as DialogTrigger,
};
