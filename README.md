# htmlq
Like jq, but for HTML. Uses CSS selectors to extract bits content from HTML files. Mozilla's MDN has a good <a href="https://developer.mozilla.org/en-US/docs/Learn/CSS/Introduction_to_CSS/Selectors">reference for CSS selector syntax</a>.


## Usage

```
$ htmlq -h
htmlq 0.0.1
Runs CSS selectors on HTML

USAGE:
    htmlq [FLAGS] [OPTIONS] <selector>...

FLAGS:
    -h, --help       Prints help information
    -t, --text       Output the contents of text elements
    -V, --version    Prints version information

OPTIONS:
    -a, --attribute <attributes>    Attributes to return from selected elements
    -f, --filename <FILE>           The input file. Defaults to stdin
    -o, --output <FILE>             The output file. Defaults to stdout

ARGS:
    <selector>...    The CSS expression to select
$
```

## Examples

### Using with cURL to find part of a page by ID

```bash
$ curl -s https://www.rust-lang.org/ | htmlq '#get-help'
<div class="four columns mt3 mt0-l" id="get-help">
        <h4>Get help!</h4>
        <ul>
          <li><a href="https://doc.rust-lang.org">Documentation</a></li>
          <li><a href="https://users.rust-lang.org">Ask a Question on the Users Forum</a></li>
          <li><a href="http://ping.rust-lang.org">Check Website Status</a></li>
        </ul>
        <div class="languages">
            <label class="hidden" for="language-footer">Language</label>
            <select id="language-footer">
                <option title="English (US)" value="en-US">English (en-US)</option>
<option title="French" value="fr">Français (fr)</option>
<option title="German" value="de">Deutsch (de)</option>

            </select>
        </div>
      </div>
```

### Find all the links in a page

```bash
$ curl -s https://www.rust-lang.org/ | htmlq -a href a
/
/tools/install
/learn
/tools
/governance
/community
https://blog.rust-lang.org/
/learn/get-started
https://blog.rust-lang.org/2019/04/25/Rust-1.34.1.html
https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html
[...]
$
```

### Get the text content of a post

```
$ curl -s https://nixos.org/nixos/about.html | htmlq  -t .main

          About NixOS

NixOS is a GNU/Linux distribution that aims to
improve the state of the art in system configuration management.  In
existing distributions, actions such as upgrades are dangerous:
upgrading a package can cause other packages to break, upgrading an
entire system is much less reliable than reinstalling from scratch,
you can’t safely test what the results of a configuration change will
be, you cannot easily undo changes to the system, and so on.  We want
to change that.  NixOS has many innovative features:

[...]
```
