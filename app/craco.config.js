const CracoSwcPlugin = require('craco-swc');

module.exports = {
  plugins: [
    {
      plugin: CracoSwcPlugin,
      // options: {
      //   swcLoaderOptions: {
      //     jsc: {
      //       externalHelpers: true,
      //     },
      //   },
      // },
    },
  ],
};
