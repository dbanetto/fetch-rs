import * as React from "react";
import "../model";

import CountHandler from "./handlers/count";
import ImageHandler from "./handlers/image";
import JsonHandler from "./handlers/json";
import KitsuHandler from "./handlers/kitsu";
import MalHandler from "./handlers/mal";
import NyaaHandler from "./handlers/nyaa";
import UrlHandler from "./handlers/url";

const typeHandlers = {
  count: CountHandler,
  image: ImageHandler,
  json: JsonHandler,
  kitsu: KitsuHandler,
  mal: MalHandler,
  nyaa: NyaaHandler,
  url: UrlHandler,
};

interface IBuilderOptions {
    edit?: boolean;
    handleUpdate?: any;
    name?: string;
}

function addHandler(type: string, builder: React.ComponentClass<IHandlerProps>) {
  typeHandlers[type] = builder;
}

function build(blob: any, type: string, options: IBuilderOptions = {}) {

  // use registered handler or default link handler
  const element: React.ComponentClass<IHandlerProps> = typeHandlers[type] || JsonHandler;

  // a reminder that handleUpdate should be set if editting
  if (options.edit && typeof(options.handleUpdate) !== "function") {
    throw new Error("Uri built to be edited but handleUpdate function is not defined");
  }

  return React.createElement(element,
    {
      blob,

      // optional properties for elements
      edit: options.edit || false,
      handleUpdate: options.handleUpdate || null,
      name: options.name || type,
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
