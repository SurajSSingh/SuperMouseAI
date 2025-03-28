import Root from "./textarea.svelte";
import type TextAreaColor from "./textarea.svelte";
import type TextAreaVariant from "./textarea.svelte";
import type TextAreaSize from "./textarea.svelte";

type FormTextareaEvent<T extends Event = Event> = T & {
  currentTarget: EventTarget & HTMLTextAreaElement;
};

type TextareaEvents = {
  blur: FormTextareaEvent<FocusEvent>;
  change: FormTextareaEvent<Event>;
  click: FormTextareaEvent<MouseEvent>;
  focus: FormTextareaEvent<FocusEvent>;
  keydown: FormTextareaEvent<KeyboardEvent>;
  keypress: FormTextareaEvent<KeyboardEvent>;
  keyup: FormTextareaEvent<KeyboardEvent>;
  mouseover: FormTextareaEvent<MouseEvent>;
  mouseenter: FormTextareaEvent<MouseEvent>;
  mouseleave: FormTextareaEvent<MouseEvent>;
  paste: FormTextareaEvent<ClipboardEvent>;
  input: FormTextareaEvent<InputEvent>;
};

export {
  type FormTextareaEvent,
  Root,
  //
  Root as Textarea,
  type TextAreaColor,
  type TextareaEvents,
  type TextAreaSize,
  type TextAreaVariant,
};
