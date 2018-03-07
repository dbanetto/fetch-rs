import { h, Component } from 'preact';
import UrlHandler from './handlers/url';
import ImageHandler from './handlers/image';
import JsonHandler from './handlers/json';
import CountHandler from './handlers/count';
import '../model';

let typeHandlers = {
  'json': JsonHandler,
  'url': UrlHandler,
  'image': ImageHandler,
  'count': CountHandler,
};

function addHandler(type: string, builder: Component<HandlerProps, void>) {
  typeHandlers[type] = builder;
}

function build(blob: any, type: string, options: any) {
  options = options || {};

  console.log("selected:" + String(type));

  // use registered handler or default link handler
  var element = typeHandlers[type] || JsonHandler;

  // a reminder that handleUpdate should be set if editting
  if (options.edit && typeof(options.handleUpdate) !== "function") {
    throw 'Uri built to be edited but handleUpdate function is not defined'
  }

  return h(element,
    {
      blob: blob,

      // optional properties for elements
      edit: options.edit || false,
      handleUpdate: options.handleUpdate,
      name: options.name,
    });
}

function listTypes(): Array<{ name: string, type: string }> {

  let types = [];
  for (var key in typeHandlers) {
    types.push({
      name: typeHandlers[key].TypeName(),
      type: key
    });
  }

  return types;
}

export default {
  build: build,
  addHandler: addHandler,
  listTypes: listTypes
};
