# htmlq
Like jq, but for HTML.

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
