# {{name}}

A simple tool for filling templates

## Display usage info

```
  tpl --help
```

## Basic usage

Use `{{!mustache}}` syntax to fill the templates up.

Provide keys and values, one by one:

```
$ tpl -key1 value1 -key2 value2
```

> Please note, keys should start with `-`.

So the template like this
```
  {{!key1}} is {{!value1}}, but {{!key2}} is {{!key2}}
```

should produce the following:
```
  key1 is value1, but key2 is value2
```

## Template file

This tool use `{{default-template-filename}}` located in the `cwd`, but you can specify a different one:

```
$ tpl --template ./different-template.tpl -key1 value1 -key2 value2
```

## Missing keys

Keys that exist in the template, but omitted will be replaced with empty string.

## Ignoring rule

The pretty common case when you need to preserve the following in your template:
```
{{!don't touch me}}
```

That's could be easily done by this:

```
{{!!don't touch me}}
```

## By the way
This `README.md` can be generated with `readmegen.sh` located in the root repo, but you must build the tool first, of course.
{{this-will-be-removed}}

Reame updated: {{build-date}}