import { h, Component } from 'preact';
import UrlHandler from './handlers/url';
import ImageHandler from './handlers/image';

let typeHandlers = {
  'url': UrlHandler,
  'image': ImageHandler,
};

function addHandler(protocol, builder) {
  uriHandlers[protocol] = builder;
}

function build(blob, type, options) {
  options = options || {};

  console.log("selected:" + String(type));

  // use registered handler or default link handler
  var element = typeHandlers[type] || UrlHandler;

  console.log("created handler with " + element.name() + " for " + String(type));

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

function listTypes() {

  let types = [];
  for (var key in typeHandlers) {
    types.push({
      name: typeHandlers[key].name(),
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
