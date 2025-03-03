import { Accordion as AccordionPrimitive } from "bits-ui";
import Content from "./accordion-content.svelte";
import Item from "./accordion-item.svelte";
import Trigger from "./accordion-trigger.svelte";
const Root = AccordionPrimitive.Root;

export {
	/** See docs at <https://next.shadcn-svelte.com/docs/components/accordion> */
	Root,
	/** See docs at <https://next.shadcn-svelte.com/docs/components/accordion> */
	Content,
	/** See docs at <https://next.shadcn-svelte.com/docs/components/accordion> */
	Item,
	/** See docs at <https://next.shadcn-svelte.com/docs/components/accordion> */
	Trigger,
	//
	/** See docs at <https://next.shadcn-svelte.com/docs/components/accordion> */
	Root as Accordion,
	/** See docs at <https://next.shadcn-svelte.com/docs/components/accordion> */
	Content as AccordionContent,
	/** See docs at <https://next.shadcn-svelte.com/docs/components/accordion> */
	Item as AccordionItem,
	/** See docs at <https://next.shadcn-svelte.com/docs/components/accordion> */
	Trigger as AccordionTrigger,
};