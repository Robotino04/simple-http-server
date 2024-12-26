A reimplementation of [this](https://github.com/Superbox2147/simple-http-server) in Rust because 300MB of dependencies for a webserver is too much.

Should support all the config options the original has and also supports `STATIC_FILE_PATH` to serve a directory other than `./`.

To deploy with this, simply place the binary in your project and name it something better. Then create the `.env` file as documented in [Superbox's version](https://github.com/Superbox2147/simple-http-server).

**Make sure you try if your build actually works!!** If it doesn't, let me know and I'll try to help.


Releases are only available for Windows, because that's what you'll need to submit your game. If you need a Linux build, you will need to compile it yourself, because I'm not smart enough to get that working. Should be pretty simple.
