import { h, Component } from 'preact';
import UriDefault from './uri/default.jsx';
import UriImage from './uri/image.jsx';

let uriHandlers = {
  'http:': UriDefault,
  'https:': UriDefault,
  'image:': UriImage,
};

function addHandler(protocol, builder) {
  uriHandlers[protocol] = builder;
}

function build(uri, options) {
  options = options || {};


  var protocol = options.protocol;
  try {
    let url = new URL(uri);

    // allows override via options which protocol to use
    protocol = protocol || url.protocol;
  } catch(err) {
    // do nothing
  }

  // use registered handler or default link handler
  var element = uriHandlers[protocol] || UriDefault;

  console.log("created url with " + element.name());

  // a reminder that handleUpdate should be set if editting
  if (options.edit && typeof(options.handleUpdate) !== "function") {
    throw 'Uri built to be edited but handleUpdate function is not defined'
  }

  return h(element,
    {
      uri: uri,

      // optional properties for elements
      edit: options.edit || false,
      handleUpdate: options.handleUpdate,
      name: options.name,
    });
}

export default { build: build, addHandler: addHandler };
