# std.html
The standard library of the brack-language for targeting to naive HTML.

## Installation
```sh
$ brack add github:brack-lang/std.html
```

## Syntax
### Heading
```brack
{std.* Hello!}
{std.** }
```

```html
<h1>Hello!</h1>
```

### Bold
```brack
[std.* Hello!]
```

```html
<span>Hello!</span>
```

### Italic
```brack
[std./ Hello!]
```

```html
<span style="">Hello!</span>
```

### List
```brack
[std.list
    Hokkaido,
    [std.list
        Aomori,
        Akita,
        Iwate,
        Yamagata,
        Miyagi,
        Fukushima
    ],
    Ibaraki
]
```

```html
<ul>
    <li>Hokkaido</li>
    <ul>
        <li>Aomori</li>
        <li>Akita</li>
        <li>Iwate</li>
        <li>Yamagata</li>
        <li>Miyagi</li>
        <li>Fukushima</li>
    </ul>
    <li>Ibaraki</li>
</ul>
```

### Quote
```brack
[std.> Hello!]
```

## LICENSE
MIT OR Apache-2.0
