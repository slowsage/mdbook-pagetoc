# mdbook-pagetoc
A mdbook plugin that provides a table of contents for each page. Uses [mdBook-pagetoc](https://github.com/JorelAli/mdBook-pagetoc).

## Show me

Sample image from [mdBook-pagetoc](https://github.com/JorelAli/mdBook-pagetoc):

![](https://raw.githubusercontent.com/JorelAli/mdBook-pagetoc/master/sample.png)


## Notes:

- Only supports the `html` renderer.

- On the first run of `mdbook build`, it creates three files `index.hbs`, `pagetoc.css` and `pagetoc.js`.

- You can customize any of them after the first run.

## Configuration:

- Add to `book.toml` config:

  ```toml
  [preprocessor.pagetoc]
  [output.html]
  additional-css = ["theme/pagetoc.css"]
  additional-js  = ["theme/pagetoc.js"]
  ```

- If using a custom `index.hbs`, the place marker `<div class="sidetoc"><nav class="pagetoc"></nav></div>` can be inserted manually inside the `<main>` tag. ie. Replace
    ```hbs
    <main>
       {{{ content }}}
    </main>
    ```

    with:


    ```hbs
    <main><div class="sidetoc"><nav class="pagetoc"></nav></div>
        {{{ content }}}
    </main>
    ```
- If not customizing, you can may want to add entries as appropriate to `.gitignore` to keep your repo clean:
  ```gitignore
  theme/index.hbs
  theme/pagetoc.css
  theme/pagetoc.js
  ```



Please see https://github.com/JorelAli/mdBook-pagetoc for more details.

-----

## Acknowledgments

- [mdBook-pagetoc](https://github.com/JorelAli/mdBook-pagetoc)

## Alternative

- [mdbook-theme](https://github.com/zjp-CN/mdbook-theme)



