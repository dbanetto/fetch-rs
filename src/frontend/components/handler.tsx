import { Component, h } from "preact";
import "../model";
import CountHandler from "./handlers/count";
import ImageHandler from "./handlers/image";
import JsonHandler from "./handlers/json";
import MalHandler from "./handlers/mal";
import NyaaHandler from "./handlers/nyaa";
import UrlHandler from "./handlers/url";

const typeHandlers = {
  count: CountHandler,
  image: ImageHandler,
  json: JsonHandler,
  mal: MalHandler,
  nyaa: NyaaHandler,
  url: UrlHandler,
};

function addHandler(type: string, builder: Component<IHandlerProps, void>) {
  typeHandlers[type] = builder;
}

function build(blob: any, type: string, options: any) {
  options = options || {};

  console.log("selected:" + String(type));

  // use registered handler or default link handler
  const element = typeHandlers[type] || JsonHandler;

  // a reminder that handleUpdate should be set if editting
  if (options.edit && typeof(options.handleUpdate) !== "function") {
    throw new Error("Uri built to be edited but handleUpdate function is not defined");
  }

  return h(element,
    {
      blob,

      // optional properties for elements
      edit: options.edit || false,
      handleUpdate: options.handleUpdate,
      name: options.name,
    });
}

function listTypes(): Array<{ name: string, type: string }> {

  const types = [];
  for (const key of Object.keys(typeHandlers)) {
    types.push({
      name: typeHandlers[key].TypeName(),
      type: key,
    });
  }

  return types;
}

export default {
  addHandler,
  build,
  listTypes,
};
