import { h, Component } from 'preact';
import UrlHandler from './handlers/url.jsx';
import ImageHandler from './handlers/image.jsx';

let typeHandlers = {
  'url': UrlHandler,
  'image': ImageHandler,
};

function addHandler(protocol, builder) {
  uriHandlers[protocol] = builder;
}

function build(blob, options) {
  options = options || {};


  var type = options.type || blob.type;

  // use registered handler or default link handler
  var element = typeHandlers[type] || UrlHandler;

  console.log("created handler with " + element.name() + " for " + type);

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

export default { build: build, addHandler: addHandler };
