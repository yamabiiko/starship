# Cấu hình

Để bắt đầu cấu hình starship, tạo tập tin sau: `~/.config/starship.toml`.

```sh
mkdir -p ~/.config && touch ~/.config/starship.toml
```

Tất cả cấu hình của starship đã xong trong tập tin này: [TOML](https://github.com/toml-lang/toml):

```toml
# Chèn một dòng trắng vào giữa các dấu nhắc lệnh
add_newline = true

# Thay thế biểu tượng "❯" trong dấu nhắc lệnh bằng "➜"
[character]                            # Tên mô đun chúng ta đang cấu hình là "character"
success_symbol = "[➜](bold green)"     # đoạn "success_symbol" đươc thiết lập thành "➜" với màu "bold green"

#Vô hiệu mô đun package, ẩn nó hoàn toàn trong dấu nhắc lệnh
[package]
disabled = true
```

Bạn thay đổi địa chỉ tệp tin cấu hình mặc định bằng biến môi trường `STARSHIP_CONFIG`:

```sh
export STARSHIP_CONFIG=~/.starship/config.toml
```

Tương đương trong PowerShell (Windows) sẽ được thêm dòng này vào `$PROFILE` của bạn:

```powershell
$ENV:STARSHIP_CONFIG = "$HOME\.starship\config.toml"
```

### Logging

Mặc định, starship logs các cảnh báo và các lỗi trong một tập tin tên là `~/.cache/starship/session_${STARSHIP_SESSION_KEY}.log`, nơi đó khoá của phiên làm việc tương ứng với thực thể terminal của bạn. Cái này, tuy nhiên có thể được thay đổi bằng cách sử dụng biến môi trường `STARSHIP_CACHE`:

```sh
export STARSHIP_CACHE=~/.starship/cache
```

Tương đương trong PowerShell (Windows) sẽ được thêm dòng này vào `$PROFILE` của bạn:

```powershell
$ENV:STARSHIP_CACHE = "$HOME\AppData\Local\Temp"
```

### Thuật ngữ

**Module**: Một thành phần trong prompt, thông tin lấy được dựa trên thông tin ngữ cảnh từ hệ điều hành của bạn. For example, the "nodejs" module shows the version of Node.js that is currently installed on your computer, if your current directory is a Node.js project.

**Variable**: Các thành phần con nhỏ hơn chứa thông tin cung cấp bởi module. For example, the "version" variable in the "nodejs" module contains the current version of Node.js.

Bằng việc quy ước, đa số các module có một tiền tố của terminal mặc định (ví dụ `via` trong "nodejs") và một khoảng trắng như là một hậu tố.

### Định dạng các chuỗi

Định dạng các chuỗi là định dạng một module với việc in ra tất cả các biến của nó. Đa số các module có một cái bắt đầu gọi là `format`, cái đó cấu hình việc hiển thị định dạng của module. Bạn có thể sử dụng các văn bản, các biến và các nhóm văn bản trong một định dạng chuỗi.

#### Biến

Một biến chứa một kí hiệu `$` theo sau bởi tên biến. Tên của một biến chỉ chứa các kí tự, các số và `_`.

Ví dụ:

- `$version` là một đính dạng chuỗi với một biến đặt tên là `version`.
- `$git_branch$git_commit` là một định dạng chuỗi với hai biến named `git_branch` và `git_commit`.
- `$git_branch $git_commit` có hai biến phân cách bằng một khoảng trắng.

#### Nhóm văn bản

Một nhóm văn bản được tạo nên bởi hai phần khác nhau.

Phần đầu tiên, cái được bao bọc trong một `[]`, là một [định dạng chuỗi](#format-strings). Bạn có thể thêm các văn bản, các biến, hoặc thậm chí các nhóm văn bản lồng nhau vào trong nó.

Phần thứ hai, cái được bao bọc trong một `()`, là một [chuỗi kiểu](#style-strings). Cái này có thể được sử dụng để quy định kiểu của phần đầu tiên.

Ví dụ:

- `[on](red bold)` sẽ in một chuỗi `on` với chữ đậm tô màu đỏ.
- `[⌘ $version](bold green)` sẽ in một biểu tượng `⌘` theo sau là nội dung của biến `version`, với chữ in đậm màu xanh lá cây.
- `[a [b](red) c](green)` sẽ in `a b c` với `b` màu đỏ, `a` và `c` màu xanh lá cây.

#### Các chuỗi kiểu

Đa số các module trong starship cho phép bạn cấu hình kiểu hiển thị của chúng. This is done with an entry (thường được gọi là `kiểu`) cái là một cuỗi cấu hình đặc biệt. Đây là vài ví dụ của các chuỗi kiểu cũng với những gì chúng làm. Cú pháp chi tiết đầy đủ, tham khảo [hướng dẫn cấu hình nâng cao](/advanced-config/).

- `"fg:green bg:blue"` thiết lập chữ màu xanh lá cây trên nền màu xanh nước biển
- `"bg:blue fg:bright-green"` thiết lập chữ màu xanh lá cây sáng trên nền màu canh nước biển
- `"bold fg:27"` thiết lập chữ đậm với [màu ANSI](https://i.stack.imgur.com/KTSQa.png) 27
- `"underline bg:#bf5700"` thiết lập chữ gạch chân trên một nền màu da cam
- `"bold italic fg:purple"` thiết lập chữa nghiêng đậm có màu tím
- `""` vô hiệu hoá tất cả các kiểu

Lưu ý rằng những style trông như thế nào sẽ được điều khiển bởi giả lập terminal của bạn. Ví dụ, một vài giả lập terminal sẽ làm sáng những màu thay vì làm đậm chữ, và một vài theme màu sử dụng cũng các giá trị cho các màu thường và màu sáng. Tương tự, để có được chữ nghiêng, terminal của bạn phải hỗ trợ các kiểu chữ nghiêng.

#### Điều kiện định dạng chuỗi

Một điều kiện định dạng chuỗi bọc trong `(` và `)` sẽ không render nếu tất cả các biến bên trong là rỗng.

Ví dụ:

- `(@$region)` sẽ không hiển thị gì nếu biến `region` là `None`, ngược lại `@` theo sao bởi giá trị của region.
- `(một vài văn bản)` sẽ không hiển thị thứ gì khi không có những biến bọc trong các dấu ngoặc.
- Khi `$all` là một shortcut cho `\[$a$b\]`, `($all)` sẽ không hiển thị chỉ khi `$a` và `$b` đều là `None`. Cái này làm việc giống như `(\[$a$b\] )`.

#### Các kí tự Escapable

Các kí hiệu sau có các sử dụng đặc biệt trong một định dạng chuỗi. Nếu bạn muốn in các kí tự sau, bạn phải đặt trước chúng kí tự backslash (`\`).

- \$
- \\
- [
- ]
- (
- )

Lưu ý rằng `toml` có [cú pháp escape riêng của nó](https://github.com/toml-lang/toml#user-content-string). Nó được khuyến nghị để sử dụng một literal string (`''`) trong cấu hình của bạn. Nếu bạn muốn sử dụng một kí tự cơ bản (`""`), chú ý đặt backslash `\` trước nó.

Ví dụ, khi bạn muốn in một kí hiệu `$` trên một dòng mới, các cấu hình sau cho `định dạng` tương đương:

```toml
# với chuỗi cơ bản
format = "\n\\$"

# với chuỗi cơ bản trong nhiều dòng
format = """

\\$"""

# với chuỗi đặc biệt
format = '''

\$'''
```

## Prompt

Cái này là danh sách các tuỳ chọn cho cấu hình prompt-wide.

### Các tuỳ chọn

| Tuỳ chọn          | Mặc định                       | Mô tả                                                                    |
| ----------------- | ------------------------------ | ------------------------------------------------------------------------ |
| `format`          | [link](#default-prompt-format) | Cấu hình định dạng của prompt.                                           |
| `scan_timeout`    | `30`                           | Timeout của starship cho việc quét các tập tin (tính theo milliseconds). |
| `command_timeout` | `500`                          | Timeout for commands executed by starship (in milliseconds).             |
| `add_newline`     | `true`                         | Chèn dòng trắng giữa các dấu nhắc lệnh.                                  |

### Ví dụ

```toml
# ~/.config/starship.toml

# Sử dụng định dạng custom
format = """
[┌───────────────────>](bold green)
[│](bold green)$directory$rust$package
[└─>](bold green) """

# Chờ 10 milliseconds để starship kiểm tra các tập tin trong đường dẫn hiện tại.
scan_timeout = 10

# Vô hiệu hóa dòng trắng tại ví trị bắt đầu của dấu nhắc lệnh
add_newline = false
```

### Định dạng prompt mặc định

Mặc định `format` được sử dụng để định nghĩa định dạng của prompt, nếu rỗng hoặc không `format` được cung cấp. Mặc định như sau:

```toml
format = "$all"

# Which is equivalent to
format = """
$username\
$hostname\
$shlvl\
$kubernetes\
$directory\
$vcsh\
$git_branch\
$git_commit\
$git_state\
$git_status\
$hg_branch\
$docker_context\
$package\
$cmake\
$dart\
$deno\
$dotnet\
$elixir\
$elm\
$erlang\
$golang\
$helm\
$java\
$julia\
$kotlin\
$nim\
$nodejs\
$ocaml\
$perl\
$php\
$purescript\
$python\
$red\
$ruby\
$rust\
$scala\
$swift\
$terraform\
$vlang\
$vagrant\
$zig\
$nix_shell\
$conda\
$memory_usage\
$aws\
$gcloud\
$openstack\
$env_var\
$crystal\
$custom\
$cmd_duration\
$line_break\
$lua\
$jobs\
$battery\
$time\
$status\
$shell\
$character"""
```

## AWS

`aws` module cho biết region và profile hiện tại của AWS. Cái này dựa trên các biến môi trường `AWS_REGION`, `AWS_DEFAULT_REGION`, và `AWS_PROFILE` với tập tin `~/.aws/config`. This module also shows an expiration timer when using temporary credentials.

When using [aws-vault](https://github.com/99designs/aws-vault) the profile is read from the `AWS_VAULT` env var and the credentials expiration date is read from the `AWS_SESSION_EXPIRATION` env var.

When using [awsu](https://github.com/kreuzwerker/awsu) the profile is read from the `AWSU_PROFILE` env var.

When using [AWSume](https://awsu.me) the profile is read from the `AWSUME_PROFILE` env var and the credentials expiration date is read from the `AWSUME_EXPIRATION` env var.

### Các tuỳ chọn

| Tuỳ chọn            | Mặc định                                                             | Mô tả                                                             |
| ------------------- | -------------------------------------------------------------------- | ----------------------------------------------------------------- |
| `format`            | `'on [$symbol($profile )(\($region\) )(\[$duration\])]($style)'` | Định dạng cho module.                                             |
| `symbol`            | `"☁️ "`                                                              | Kí hiệu sử dụng hiển thị trước profile AWS hiện tại.              |
| `region_aliases`    |                                                                      | Bảng của các region alias để hiển thị ngoài tên AWS.              |
| `style`             | `"bold yellow"`                                                      | Kiểu cho module.                                                  |
| `expiration_symbol` | `X`                                                                  | The symbol displayed when the temporary credentials have expired. |
| `disabled`          | `false`                                                              | Disables the `AWS` module.                                        |

### Các biến

| Biến      | Ví dụ            | Mô tả                                       |
| --------- | ---------------- | ------------------------------------------- |
| region    | `ap-northeast-1` | Region AWS hiện tại                         |
| profile   | `astronauts`     | Profile AWS hiện tại                        |
| duration  | `2h27m20s`       | The temporary credentials validity duration |
| symbol    |                  | Giá trị ghi đè tuỳ chọn `symbol`            |
| style\* |                  | Giá trị ghi đè của `style`                  |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Các vị dụ

#### Hiển thị mọi thứ

```toml
# ~/.config/starship.toml

[aws]
format = 'on [$symbol($profile )(\($region\) )]($style)'
style = "bold blue"
symbol = "🅰 "
[aws.region_aliases]
ap-southeast-2 = "au"
us-east-1 = "va"
```

#### Hiển thị region

```toml
# ~/.config/starship.toml

[aws]
format = "on [$symbol$region]($style) "
style = "bold blue"
symbol = "🅰 "
[aws.region_aliases]
ap-southeast-2 = "au"
us-east-1 = "va"
```

#### Hiển thị profile

```toml
# ~/.config/starship.toml

[aws]
format = "on [$symbol$profile]($style) "
style = "bold blue"
symbol = "🅰 "
```

## Battery

`battery` module cho biết cách sạc pin của thiết bị là gì và tình trạng sạc hiện tại của nó. Module chỉ được nhìn thấy khi pin của thiết bị dưới 10%.

### Các tuỳ chọn

| Tuỳ chọn             | Mặc định                          | Mô tả                                                    |
| -------------------- | --------------------------------- | -------------------------------------------------------- |
| `full_symbol`        | `" "`                            | Kí hiệu cho biết khi pin đầy.                            |
| `charging_symbol`    | `" "`                            | Kí hiệu cho biết khi ping đang sạc.                      |
| `discharging_symbol` | `" "`                            | Kí hiệu cho biết khi pin đang không sạc.                 |
| `unknown_symbol`     | `" "`                            | Kí hiệu cho biết khi trạng thái pin không được xác định. |
| `empty_symbol`       | `" "`                            | Kí hiệu cho biết khi hết pin.                            |
| `format`             | `"[$symbol$percentage]($style) "` | Định dạng cho module.                                    |
| `display`            | [link](#battery-display)          | Ngưỡng hiển thị và kiểu cho module.                      |
| `disabled`           | `false`                           | Vô hiệu `battery` module.                                |

### Ví dụ

```toml
# ~/.config/starship.toml

[battery]
full_symbol = "🔋 "
charging_symbol = "⚡️ "
discharging_symbol = "💀 "
```

### Hiển thị pin

The `display` configuration option is used to define when the battery indicator should be shown (threshold), which symbol would be used (symbol), and what it would like (style). Nếu `display` không được cung cấp. Mặc định như sau:

```toml
[[battery.display]]
threshold = 10
style = "bold red"
```

The default value for the `charging_symbol` and `discharging_symbol` option is respectively the value of `battery`'s `charging_symbol` and `discharging_symbol` option.

#### Các tuỳ chọn

Tuỳ chọn `display` là một mảng của của bảng sau.

| Tuỳ chọn             | Mặc định   | Mô tả                                                                                                     |
| -------------------- | ---------- | --------------------------------------------------------------------------------------------------------- |
| `threshold`          | `10`       | Cận trên cho tuỳ chọn hiển thị.                                                                           |
| `style`              | `bold red` | Kiểu sử dụng nếu tuỳ chọn hiển thị được sử dụng bên trong.                                                |
| `charging_symbol`    | `-`        | Optional symbol displayed if display option is in use, defaults to battery's `charging_symbol` option.    |
| `discharging_symbol` | `-`        | Optional symbol displayed if display option is in use, defaults to battery's `discharging_symbol` option. |

#### Ví dụ

```toml
[[battery.display]]  # "bold red" style and discharging_symbol when capacity is between 0% and 10%
threshold = 10
style = "bold red"

[[battery.display]]  # "bold yellow" style and 💦 symbol when capacity is between 10% and 30%
threshold = 30
style = "bold yellow"
discharging_symbol = 💦

# when capacity is over 30%, the battery indicator will not be displayed

```

## Character

Module `character` cho biết một kí tự (thường là một mũi tên) bên cạnh nơi văn bản được nhập trong terminal của bạn.

Kí tự sẽ nói cho bạn câu lệnh cuối liệu thành công hay thất bại. Nó có thể làm điều này bằng hai cách:

- thay đổi màu(`đỏ`/`xanh lá`)
- thay đổi hình dạng (`❯`/`✖`)

Mặc định, nó chỉ thay đổi màu. If you also want to change its shape take a look at [this example](#with-custom-error-shape).

::: warning `error_symbol` không được hỗ trợ trên elvish shell. :::

### Các tuỳ chọn

| Tuỳ chọn         | Mặc định            | Mô tả                                                                                |
| ---------------- | ------------------- | ------------------------------------------------------------------------------------ |
| `format`         | `"$symbol "`        | Định dạng chuỗi sử dụng trước văn bản nhập vào.                                      |
| `success_symbol` | `"[❯](bold green)"` | Định dạng chuỗi sửa dụng trước văn bản nhập vào nếu câu lệnh trước đó đã thành công. |
| `error_symbol`   | `"[❯](bold red)"`   | Định dạng chuỗi sửa dụng trước văn bản nhập vào nếu câu lệnh trước đó đã thất bại.   |
| `vicmd_symbol`   | `"[❮](bold green)"` | Định dạng chuỗi sửa dụng trước văn bản nhập vào nếu shell trong chế độ vim normal.   |
| `disabled`       | `false`             | Vô hiệu module `character`.                                                          |

### Các biến

| Biến   | Ví dụ | Mô tả                                                                         |
| ------ | ----- | ----------------------------------------------------------------------------- |
| symbol |       | Một phản ánh của một trong `success_symbol`, `error_symbol` or `vicmd_symbol` |

### Các vị dụ

#### Có tuỳ chỉnh hình dạng lỗi

```toml
# ~/.config/starship.toml

[character]
success_symbol = "[➜](bold green) "
error_symbol = "[✗](bold red) "
```

#### Không có tuỳ chỉnh hình dạng lỗi

```toml
# ~/.config/starship.toml

[character]
success_symbol = "[➜](bold green) "
error_symbol = "[➜](bold red) "
```

#### Có tuỳ chỉnh hình dạng vim

```toml
# ~/.config/starship.toml

[character]
vicmd_symbol = "[V](bold green) "
```

## CMake

The `cmake` module shows the currently installed version of [CMake](https://cmake.org/). Mặc định module sẽ được kích hoạt nếu thoả mãn bất kì điều kiện nào dưới đây:

- Đường dẫn hiện tại chứa một tập tin `CmakeLists.txt`
- Đường dẫn hiện tại chứa một tập tin `CMakeCache.txt`

### Các tuỳ chọn

| Tuỳ chọn            | Mặc định                               | Mô tả                                                                     |
| ------------------- | -------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`   | Định dạng cho module.                                                     |
| `version_format`    | `"v${raw}"`                            | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"△ "`                                 | Kí hiệu sử dụng trước phiên bản của cmake.                                |
| `detect_extensions` | `[]`                                   | Những tiện ích mở rộng nào sẽ kích hoạt mô-đun này                        |
| `detect_files`      | `["CMakeLists.txt", "CMakeCache.txt"]` | Tên tệp nào sẽ kích hoạt mô-đun này                                       |
| `detect_folders`    | `[]`                                   | Thư mục nào sẽ kích hoạt mô-đun này                                       |
| `style`             | `"bold blue"`                          | Kiểu cho module.                                                          |
| `disabled`          | `false`                                | Vô hiệu hoá `cmake` module.                                               |

### Các biến

| Biến      | Ví dụ     | Mô tả                            |
| --------- | --------- | -------------------------------- |
| version   | `v3.17.3` | Phiên bản của cmake              |
| symbol    |           | Giá trị ghi đè tuỳ chọn `symbol` |
| style\* |           | Giá trị ghi đè của `style`       |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

## Command Duration

Module `cmd_duration`. cho biết câu lệnh cuối cùng thực thi trong bao lâu. Module sẽ được hiện chỉ khi câu lệnh lấy nhiều hơn 2 giây, hoặc giá trị cấu hình `min_time`, nếu nó tồn tại.

::: cảnh báo Không thể hook DEBUG trap trong Bash

Nếu bạn đang chạy Starship trong `bash`, không thể hook `DEBUG` trap sau khi chạy `eval $(starship init $0)`, hoặc module này **sẽ** ngắt.

:::

Người dùng Bash, những người cần chức năng giống preexec có thể sử dụng [rcaloras's bash_preexec framework](https://github.com/rcaloras/bash-preexec). Đơn giản là định nghĩa các mảng `preexec_functions` và `precmd_functions` trước khi chạy `eval $(starship init $0)`, và sau đó thực thi như bình thường.

### Các tuỳ chọn

| Tuỳ chọn             | Mặc định                      | Mô tả                                                                  |
| -------------------- | ----------------------------- | ---------------------------------------------------------------------- |
| `min_time`           | `2_000`                       | Khoảng thời gian ngắn nhất để hiện thời gian (tính bằng milliseconds). |
| `show_milliseconds`  | `false`                       | Hiện milliseconds.                                                     |
| `format`             | `"took [$duration]($style) "` | Định dạng cho module.                                                  |
| `style`              | `"bold yellow"`               | Kiểu cho module.                                                       |
| `disabled`           | `false`                       | Vô hiệu module `cmd_duration`.                                         |
| `show_notifications` | `false`                       | Hiện thông báo desktop khi câu lệnh hoàn thành.                        |
| `min_time_to_notify` | `45_000`                      | Khoảng thời gian ngắn nhất để thông báo (tính bằng milliseconds).      |

::: thử thuật

Hiện thông báo desktop yêu cầu starship được built với sự hỗ trợ của `rust-notify`. Bạn kiểm tra nếu starship hỗ trợ các thông báo bằng cách chạy `STARSHIP_LOG=debug starship module cmd_duration -d 60000` khi `show_notifications` được thiết lập là `true`.

:::

### Các biến

| Biến      | Ví dụ    | Mô tả                                 |
| --------- | -------- | ------------------------------------- |
| duration  | `16m40s` | Thời gian nó lấy để thực thi câu lệnh |
| style\* |          | Giá trị ghi đè của `style`            |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[cmd_duration]
min_time = 500
format = "underwent [$duration](bold yellow)"
```

## Conda

Module `conda` cho biết môi trường conda hiện tại, nếu `$CONDA_DEFAULT_ENV` được thiết lập.

::: thử thuật

Cái này không loại bỏ conda's prompt mà nó sở hữu, bạn có thể muốn chạy `conda config --set changeps1 False`.

:::

### Các tuỳ chọn

| Tuỳ chọn            | Mặc định                               | Mô tả                                                                                                                                                                                                       |
| ------------------- | -------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `truncation_length` | `1`                                    | Số lượng đường dẫn của biến môi trường nên được cắt bớt, nếu biến môi trường được tạo thông qua via `conda create -p [path]`. `0` nghĩa là không cắt bớt. Cũng thấy trong module [`directory`](#directory). |
| `symbol`            | `"🅒 "`                                 | Kí hiệu sử dụng trước tên biến môi trường.                                                                                                                                                                  |
| `style`             | `"bold green"`                         | Kiểu cho module.                                                                                                                                                                                            |
| `format`            | `"via [$symbol$environment]($style) "` | Định dạng cho module.                                                                                                                                                                                       |
| `ignore_base`       | `true`                                 | Bỏ qua biến môi trường `base` khi đã kích hoạt.                                                                                                                                                             |
| `disabled`          | `false`                                | Vô hiệu module `conda`.                                                                                                                                                                                     |

### Các biến

| Biến        | Ví dụ        | Mô tả                              |
| ----------- | ------------ | ---------------------------------- |
| environment | `astronauts` | Biến môi trường hiện tại của conda |
| symbol      |              | Giá trị ghi đè tuỳ chọn `symbol`   |
| style\*   |              | Giá trị ghi đè của `style`         |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[conda]
format = "[$symbol$environment](dimmed green) "
```

## Crystal

The `crystal` module shows the currently installed version of [Crystal](https://crystal-lang.org/). Mặc định module sẽ được hiển thị nếu có bất kì điều kiện nào dưới đây thoả mãn:

- Đường dẫn hiện tại chứa một tập tin `shard.yml`
- Đường dẫn hiện tại chứa một tập tin `.cr`

### Options

| Tuỳ chọn            | Mặc định                             | Mô tả                                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `symbol`            | `"🔮 "`                               | Kí hiệu sử dụng trước phiên bản hiển thị của crystal.                     |
| `format`            | `"via [$symbol($version )]($style)"` | Định dạng cho module.                                                     |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `style`             | `"bold red"`                         | Kiểu cho module.                                                          |
| `detect_extensions` | `["cr"]`                             | Những tiện ích mở rộng nào sẽ kích hoạt mô-đun này.                       |
| `detect_files`      | `["shard.yml"]`                      | Tên tệp nào sẽ kích hoạt mô-đun này.                                      |
| `detect_folders`    | `[]`                                 | Những thư mục nào sẽ kích hoạt mô-đun này.                                |
| `disabled`          | `false`                              | Vô hiệu hoá module `crystal`.                                             |

### Các biến

| Biến      | Ví dụ     | Mô tả                            |
| --------- | --------- | -------------------------------- |
| version   | `v0.32.1` | Phiên bản của `crystal`          |
| symbol    |           | Giá trị ghi đè tuỳ chọn `symbol` |
| style\* |           | Giá trị ghi đè của `style`       |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[crystal]
format = "via [✨ $version](bold blue) "
```

## Dart

The `dart` module shows the currently installed version of [Dart](https://dart.dev/). Mặc định module sẽ được hiển thị nếu có bất kì điều kiện nào dưới đây thoả mãn:

- Đường dẫn hiện tại chứa một tập tin với phần mở rộng `.dart`
- Đường dẫn hiện tại chứa một đường dẫn `.dart_tool`
- Đường dẫn hiện tại chứa một tệp tin `pubspec.yaml`, `pubspec.yml` hoặc `pubspec.lock`

### Các tuỳ chọn

| Tuỳ chọn            | Mặc định                                          | Mô tả                                                                     |
| ------------------- | ------------------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`              | Định dạng cho module.                                                     |
| `version_format`    | `"v${raw}"`                                       | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🎯 "`                                            | Một chuỗi định dạng hiển thị biểu tượng của Dart                          |
| `detect_extensions` | `["dart"]`                                        | Những tiện ích mở rộng nào sẽ kích hoạt mô-đun này.                       |
| `detect_files`      | `["pubspec.yaml", "pubspec.yml", "pubspec.lock"]` | Tên tệp nào sẽ kích hoạt mô-đun này.                                      |
| `detect_folders`    | `[".dart_tool"]`                                  | Những thư mục nào sẽ kích hoạt mô-đun này.                                |
| `style`             | `"bold blue"`                                     | Kiểu cho module.                                                          |
| `disabled`          | `false`                                           | Vô hiệu `dart` module.                                                    |

### Các biến

| Biến      | Ví dụ    | Mô tả                            |
| --------- | -------- | -------------------------------- |
| version   | `v2.8.4` | Phiên bản của `dart`             |
| symbol    |          | Giá trị ghi đè tuỳ chọn `symbol` |
| style\* |          | Giá trị ghi đè của `style`       |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[dart]
format = "via [🔰 $version](bold red) "
```

## Deno

The `deno` module shows you your currently installed version of [Deno](https://deno.land/). Mặc định module sẽ được hiển thị nếu có bất kì điều kiện nào dưới đây thoả mãn:
- The current directory contains a `mod.ts`, `mod.js`, `deps.ts` or `deps.js` file

### Các tuỳ chọn

| Tuỳ chọn            | Mặc định                                     | Mô tả                                                                     |
| ------------------- | -------------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`         | Định dạng cho module.                                                     |
| `version_format`    | `"v${raw}"`                                  | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🦕 "`                                       | A format string representing the symbol of Deno                           |
| `detect_extensions` | `[]`                                         | Những tiện ích mở rộng nào sẽ kích hoạt mô-đun này.                       |
| `detect_files`      | `["mod.ts", "mod.js", "deps.ts", "deps.js"]` | Tên tệp nào sẽ kích hoạt mô-đun này.                                      |
| `detect_folders`    | `[]`                                         | Những thư mục nào sẽ kích hoạt mô-đun này.                                |
| `style`             | `"green bold"`                               | Kiểu cho module.                                                          |
| `disabled`          | `false`                                      | Disables the `deno` module.                                               |

### Các biến

| Biến      | Ví dụ    | Mô tả                            |
| --------- | -------- | -------------------------------- |
| version   | `v1.8.3` | The version of `deno`            |
| symbol    |          | Giá trị ghi đè tuỳ chọn `symbol` |
| style\* |          | Giá trị ghi đè của `style`       |

### Ví dụ

```toml
# ~/.config/starship.toml

[deno]
format = "via [🦕 $version](green bold) "
```

## Đường dẫn

`directory` module hiển thị đường dẫn thư mục hiện hành của bạn,, cắt ngắn ba thư mục cha. Đường dẫn của bạn cũng sẽ được cắt ngắn tới đường dẫn gốc của git repo hiện tại của bạn.

Khi sử dụng fish style pwd option, thay vì ẩn đường dẫn được rút gọn, bạn sẽ thấy một tên ngắn cho mỗi thư mục dựa trên số bạn cho phép trng tùy chọn.

Cho ví dụ, `~/Dev/Nix/nixpkgs/pkgs` nơi `nixpkgs` là gốc của repo, và tuỳ chọn thiết lập sang `1`. Bây giờ bạn sẽ thấy `~/D/N/nixpkgs/pkgs`, trong khi trước nó là `nixpkgs/pkgs`.

### Các tuỳ chọn

| Tuỳ chọn            | Mặc định                                           | Mô tả                                                              |
| ------------------- | -------------------------------------------------- | ------------------------------------------------------------------ |
| `truncation_length` | `3`                                                | Số lượng thư mục cha của thư mục hiện tại nên được rút gọn.        |
| `truncate_to_repo`  | `true`                                             | Có hoặc không rút gọn đường dẫn gốc của git repo hiện tại của bạn. |
| `format`            | `"[$path]($style)[$read_only]($read_only_style) "` | Định dạng cho module.                                              |
| `style`             | `"bold cyan"`                                      | Kiểu cho module.                                                   |
| `disabled`          | `false`                                            | Vô hiệu mô đun `directory`.                                        |
| `read_only`         | `"🔒"`                                              | Biểu tượng để nhận biết thư mục hiện tại là chỉ đọc.               |
| `read_only_style`   | `"red"`                                            | Style cho biểu tượng chỉ đọc.                                      |
| `truncation_symbol` | `""`                                               | Biểu tượng tiền tố cho các đường dẫn rút gọn. ví dụ: "…/"          |
| `home_symbol`       | `"~"`                                              | Biểu tượng nhận biết thư mục home.                                 |

<details>
<summary>Mô đun này có một vài tùy chọn nâng cao để điều khiển cách thư mục được hiển thị.</summary>

| Tùy chọn nâng cao           | Mặc định | Mô tả                                                                                                                                                                  |
| --------------------------- | -------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `substitutions`             |          | A table of substitutions to be made to the path.                                                                                                                       |
| `fish_style_pwd_dir_length` | `0`      | The number of characters to use when applying fish shell pwd path logic.                                                                                               |
| `use_logical_path`          | `true`   | If `true` render the logical path sourced from the shell via `PWD` or `--logical-path`. If `false` instead render the physical filesystem path with symlinks resolved. |

`substitutions` allows you to define arbitrary replacements for literal strings that occur in the path, for example long network prefixes or development directories (i.e. Java). Note that this will disable the fish style PWD.

```toml
[directory.substitutions]
"/Volumes/network/path" = "/net"
"src/com/long/java/path" = "mypath"
```

`fish_style_pwd_dir_length` interacts with the standard truncation options in a way that can be surprising at first: if it's non-zero, the components of the path that would normally be truncated are instead displayed with that many characters. For example, the path `/built/this/city/on/rock/and/roll`, which would normally be displayed as as `rock/and/roll`, would be displayed as `/b/t/c/o/rock/and/roll` with `fish_style_pwd_dir_length = 1`--the path components that would normally be removed are displayed with a single character. For `fish_style_pwd_dir_length = 2`, it would be `/bu/th/ci/on/rock/and/roll`.

</details>

### Các biến

| Biến      | Ví dụ                 | Mô tả                      |
| --------- | --------------------- | -------------------------- |
| path      | `"D:/Projects"`       | Đường dẫn thư mục hiện tại |
| style\* | `"black bold dimmed"` | Giá trị ghi đè của `style` |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[directory]
truncation_length = 8
truncation_symbol = "…/"
```

## Docker Context

Mô đun `docker_context` hiển thị [Docker context](https://docs.docker.com/engine/context/working-with-contexts/) hiện tại được kích hoạt nếu nó không được thiết lập `mặc định`.

### Các tuỳ chọn

| Tuỳ chọn            | Mặc định                                                      | Mô tả                                                                                    |
| ------------------- | ------------------------------------------------------------- | ---------------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol$context]($style) "`                            | Định dạng cho module.                                                                    |
| `symbol`            | `"🐳 "`                                                        | Biểu tượng sử dụng để hiển thị trước Docker context.                                     |
| `only_with_files`   | `true`                                                        | Chỉ hiển thị khi có một tệp tin khớp                                                     |
| `detect_extensions` | `[]`                                                          | Các mở rộng nào nên kích hoạt mô đun này (cần `only_with_files` thiết lập là true).      |
| `detect_files`      | `["docker-compose.yml", "docker-compose.yaml", "Dockerfile"]` | Tên tệp tin nào nên kích hoạt mô đun này (cần `only_with_files` được thiết lập là true). |
| `detect_folders`    | `[]`                                                          | Thư mục nào nên kích hoạt mô đun này (cần `only_with_files` được thiết lập là true).     |
| `style`             | `"blue bold"`                                                 | Kiểu cho module.                                                                         |
| `disabled`          | `false`                                                       | Vô hiệu mô đun `docker_context`.                                                         |

### Các biến

| Biến      | Ví dụ          | Mô tả                            |
| --------- | -------------- | -------------------------------- |
| context   | `test_context` | Docker context hiện tại          |
| symbol    |                | Giá trị ghi đè tuỳ chọn `symbol` |
| style\* |                | Giá trị ghi đè của `style`       |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[docker_context]
format = "via [🐋 $context](blue bold)"
```

## Dotnet

The `dotnet` module shows the relevant version of the [.NET Core SDK](https://dotnet.microsoft.com/) for the current directory. Nếu SDK được ghim trong thư mục hiện tại, phiên bản ghim đó được hiển thị. Ngược lại, mô đun hiển thị phiên bản cuối cùng của SDK được cài đặt.

Mặc định, mô đun này sẽ chỉ được hiển thị trong dấu nhắc lệnh của bạn khi một hoặc nhiều tệp tin dưới đây xuất hiện trong thư mục hiện tại:

- `global.json`
- `project.json`
- `Directory.Build.props`
- `Directory.Build.targets`
- `Packages.props`
- `*.sln`
- `*.csproj`
- `*.fsproj`
- `*.xproj`

Bạn cũng sẽ cần cài đặt .NET Core SDK đúng cách để sử dụng một cách chính xác.

Mô đun này sử dụng cơ chế của bản thân để phát hiện phiên bản của chính nó. Thông thường, nó nhanh gấp đôi nếu chạy `dotnet --version`, nhưng nó có thể hiện sai phiên bản nếu dự án .NET của bạn có một cấu trúc thư mục bất thường. Nếu độ chính xác quan trọng hơn tốc độ, bạn có thể vô hiệu cơ chế bằng cài đặt `heuristic = false` trong các tùy chọn mô đun.

Mô đun cũng sẽ hiện Target Framework Moniker (<https://docs.microsoft.com/en-us/dotnet/standard/frameworks#supported-target-framework-versions>) khi có một tệp tin csproj trong thư mục hiện tại.

### Các tuỳ chọn

| Tuỳ chọn            | Mặc định                                                                                                | Mô tả                                                                     |
| ------------------- | ------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `"[$symbol($version )(🎯 $tfm )]($style)"`                                                               | Định dạng cho module.                                                     |
| `version_format`    | `"v${raw}"`                                                                                             | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `".NET "`                                                                                               | Biểu tượng sử dụng để hiển thị trước phiên bản của dotnet.                |
| `heuristic`         | `true`                                                                                                  | Sử dụng phiên bản phát hiện thông minh hơn.                               |
| `detect_extensions` | `["sln", "csproj", "fsproj", "xproj"]`                                                                  | Những tiện ích mở rộng nào sẽ kích hoạt mô-đun này.                       |
| `detect_files`      | `["global.json", "project.json", "Directory.Build.props", "Directory.Build.targets", "Packages.props"]` | Tên tệp nào sẽ kích hoạt mô-đun này.                                      |
| `detect_folders`    | `[]`                                                                                                    | Những thư mục nào nên kích hoạt các mô đun này.                           |
| `style`             | `"bold blue"`                                                                                           | Kiểu cho module.                                                          |
| `disabled`          | `false`                                                                                                 | Vô hiệu mô đun `dotnet`.                                                  |

### Các biến

| Biến      | Ví dụ            | Mô tả                                                         |
| --------- | ---------------- | ------------------------------------------------------------- |
| version   | `v3.1.201`       | Phiên bản của `dotnet` sdk                                    |
| tfm       | `netstandard2.0` | Target Framework Monike của dự án hiện tại đang được nhắm đến |
| symbol    |                  | Giá trị ghi đè tuỳ chọn `symbol`                              |
| style\* |                  | Giá trị ghi đè của `style`                                    |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[dotnet]
symbol = "🥅 "
style = "green"
heuristic = false
```

## Elixir

The `elixir` module shows the currently installed version of [Elixir](https://elixir-lang.org/) and [Erlang/OTP](https://erlang.org/doc/). Mặc định module sẽ được hiển thị nếu có bất kì điều kiện nào dưới đây thoả mãn:

- Đường dẫn hiện tại chứa một tập tin `mix.exs`.

### Các tuỳ chọn

| Tuỳ chọn            | Mặc định                                                    | Mô tả                                                                     |
| ------------------- | ----------------------------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version \(OTP $otp_version\) )]($style)'` | Định dạng cho module elixir.                                              |
| `version_format`    | `"v${raw}"`                                                 | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"💧 "`                                                      | Kí hiệu sử dụng trước phiên bản hiển thị của Elixir/Erlang.               |
| `detect_extensions` | `[]`                                                        | Những tiện ích mở rộng nào sẽ kích hoạt mô-đun này.                       |
| `detect_files`      | `["mix.exs"]`                                               | Tên tệp nào sẽ kích hoạt mô-đun này.                                      |
| `detect_folders`    | `[]`                                                        | Những thư mục nào nên kích hoạt các mô đun này.                           |
| `style`             | `"bold purple"`                                             | Kiểu cho module.                                                          |
| `disabled`          | `false`                                                     | Vô hiệu mô đun `elixir`.                                                  |

### Các biến

| Biến        | Ví dụ   | Mô tả                            |
| ----------- | ------- | -------------------------------- |
| version     | `v1.10` | Phiên bản của `elixir`           |
| otp_version |         | Phiên bản otp của `elixir`       |
| symbol      |         | Giá trị ghi đè tuỳ chọn `symbol` |
| style\*   |         | Giá trị ghi đè của `style`       |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[elixir]
symbol = "🔮 "
```

## Elm

The `elm` module shows the currently installed version of [Elm](https://elm-lang.org/). Mặc định module sẽ được hiển thị nếu có bất kì điều kiện nào dưới đây thoả mãn:

- Đường dẫn hiện tại chứa một tập tin `elm.json`
- Đường dẫn hiện tại chứa một tập tin `elm-package.json`
- Đường dẫn hiện tại chứa một tệp tin `.elm-version`
- Đường dẫn hiện tại chứa một thư mục `elm-stuff`
- Đường dẫn hiện tại chứa một tập tin `*.elm`

### Các tuỳ chọn

| Tuỳ chọn            | Mặc định                                           | Mô tả                                                                     |
| ------------------- | -------------------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`               | Định dạng cho module.                                                     |
| `version_format`    | `"v${raw}"`                                        | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🌳 "`                                             | Một format string đại diện cho biểu tượng của Elm.                        |
| `detect_extensions` | `["elm"]`                                          | Những tiện ích mở rộng nào sẽ kích hoạt mô-đun này.                       |
| `detect_files`      | `["elm.json", "elm-package.json", ".elm-version"]` | Tên tệp nào sẽ kích hoạt mô-đun này.                                      |
| `detect_folders`    | `["elm-stuff"]`                                    | Những thư mục nào nên kích hoạt các mô đun này.                           |
| `style`             | `"cyan bold"`                                      | Kiểu cho module.                                                          |
| `disabled`          | `false`                                            | Vô hiệu mô đun `elm`.                                                     |

### Các biến

| Biến      | Ví dụ     | Mô tả                            |
| --------- | --------- | -------------------------------- |
| version   | `v0.19.1` | Phiên bản của `elm`              |
| symbol    |           | Giá trị ghi đè tuỳ chọn `symbol` |
| style\* |           | Giá trị ghi đè của `style`       |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[elm]
format = "via [ $version](cyan bold) "
```

## Biến môi trường

Mô đun `env_var` hiển thị giá trị hiện tại của biến môi trường được chọn. Mô đun sẽ được hiển thị chỉ khi bất kì điều kiện nào sau đây thỏa mãn:

- Tùy chọn `variable` khớp với mootjj biến môi trường tồn tại
- Tùy chọn `variable` không được định nghĩa, nhưng tùy chọn `default` là

### Các tuỳ chọn

| Tuỳ chọn   | Mặc định                       | Mô tả                                                                    |
| ---------- | ------------------------------ | ------------------------------------------------------------------------ |
| `symbol`   |                                | Biểu tượng sử dụng để hiển thị trước giá trị của biến.                   |
| `variable` |                                | Biến môi trường được hiển thị.                                           |
| `default`  |                                | Giá trị mặc định được hiển thị khi biến được chọn không được định nghĩa. |
| `format`   | `"with [$env_value]($style) "` | Định dạng cho module.                                                    |
| `disabled` | `false`                        | Vô hiệu `env_var`.                                                       |

### Các biến

| Biến      | Ví dụ                                     | Mô tả                                           |
| --------- | ----------------------------------------- | ----------------------------------------------- |
| env_value | `Windows NT` (nếu _variable_ sẽ là `$OS`) | Giá trị biến môi trường của tùy chọn `variable` |
| symbol    |                                           | Giá trị ghi đè tuỳ chọn `symbol`                |
| style\* | `black bold dimmed`                       | Giá trị ghi đè của `style`                      |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[env_var]
variable = "SHELL"
default = "unknown shell"
```

## Erlang

The `erlang` module shows the currently installed version of [Erlang/OTP](https://erlang.org/doc/). Mặc định module sẽ được hiển thị nếu có bất kì điều kiện nào dưới đây thoả mãn:

- Đường dẫn hiện tại chứa một tập tin `rebar.config`.
- Đường dẫn hiện tại chứa một tập tin `erlang.mk`.

### Các tuỳ chọn

| Tuỳ chọn            | Mặc định                             | Mô tả                                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Định dạng cho module.                                                     |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `" "`                               | Biểu tượng sử dụng để hiển thị trước phiên bản của erlang.                |
| `style`             | `"bold red"`                         | Kiểu cho module.                                                          |
| `detect_extensions` | `[]`                                 | Những tiện ích mở rộng nào sẽ kích hoạt mô-đun này.                       |
| `detect_files`      | `["rebar.config", "elang.mk"]`       | Tên tệp nào sẽ kích hoạt mô-đun này.                                      |
| `detect_folders`    | `[]`                                 | Những thư mục nào nên kích hoạt các mô đun này.                           |
| `disabled`          | `false`                              | Vô hiệu mô đun `erlang`.                                                  |

### Các biến

| Biến      | Ví dụ     | Mô tả                            |
| --------- | --------- | -------------------------------- |
| version   | `v22.1.3` | The version of `erlang`          |
| symbol    |           | Giá trị ghi đè tuỳ chọn `symbol` |
| style\* |           | Giá trị ghi đè của `style`       |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[erlang]
format = "via [e $version](bold red) "
```

## Google Cloud (`gcloud`)

Mô đun `gcloud` hiển thị cấu hình hiện tại của [`gcloud`](https://cloud.google.com/sdk/gcloud) CLI. Cái này dựa trên tập tin `~/.config/gcloud/active_config`, `~/.config/gcloud/configurations/config_{CONFIG NAME}` và biến môi trường `CLOUDSDK_CONFIG`.

### Các tuỳ chọn

| Tuỳ chọn         | Mặc định                                                   | Mô tả                                                             |
| ---------------- | ---------------------------------------------------------- | ----------------------------------------------------------------- |
| `format`         | `'on [$symbol$account(@$domain)(\($region\))]($style) '` | Định dạng cho module.                                             |
| `symbol`         | `"☁️  "`                                                   | Kí hiệu sử dụng hiển thị trước profile GCP hiện tại.              |
| `region_aliases` |                                                            | Bảng ánh xạ của các bí danh của region để hiển thị ngoài tên GCP. |
| `style`          | `"bold blue"`                                              | Kiểu cho module.                                                  |
| `disabled`       | `false`                                                    | Vô hiệu mô đun `gcloud`.                                          |

### Các biến

| Biến      | Ví dụ         | Mô tả                                                                |
| --------- | ------------- | -------------------------------------------------------------------- |
| region    | `us-central1` | Region GCP hiện tại                                                  |
| account   | `foo`         | Profile hiện tại của GCP                                             |
| domain    | `example.com` | The current GCP profile domain                                       |
| project   |               | Dự án hiện tại của GCP                                               |
| active    | `default`     | Tên cấu hình có hiệu lực viết trong `~/.config/gcloud/active_config` |
| symbol    |               | Giá trị ghi đè tuỳ chọn `symbol`                                     |
| style\* |               | Giá trị ghi đè của `style`                                           |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Các vị dụ

#### Hiển thị tài khoản và dự án

```toml
# ~/.config/starship.toml

[gcloud]
format = 'on [$symbol$account(@$domain)(\($project\))]($style) '
```

#### Chỉ hiển thị tên cấu hình hiệu lực

```toml
# ~/.config/starship.toml

[gcloud]
format = "[$symbol$active]($style) "
style = "bold yellow"
```

#### Hiển thị tài khoản và bí danh khu vực

```toml
# ~/.config/starship.toml

[gcloud]
symbol = "️🇬️ "
[gcloud.region_aliases]
us-central1 = "uc1"
asia-northeast1 = "an1"
```

## Git Branch

Mô đun `git_branch` hiển thị nhánh hiệu lực của repo trong thư mục hiện tại của bạn.

### Các tuỳ chọn

| Tuỳ chọn             | Mặc định                         | Mô tả                                                                                                 |
| -------------------- | -------------------------------- | ----------------------------------------------------------------------------------------------------- |
| `always_show_remote` | `false`                          | Hiển thị tên nhánh remote tracking, thậm chí nếu nó bằng với tên nhánh local.                         |
| `format`             | `"on [$symbol$branch]($style) "` | Định dạng cho module. Sử dụng `"$branch"` để tham chiếu tới tên nhánh hiện tại.                       |
| `symbol`             | `" "`                           | Một chuỗi định dạng hiển thị biểu tượng của nhánh git.                                                |
| `style`              | `"bold purple"`                  | Kiểu cho module.                                                                                      |
| `truncation_length`  | `2^63 - 1`                       | Truncates a git branch to `N` graphemes.                                                              |
| `truncation_symbol`  | `"…"`                            | Biểu tượng sử dụng để nhận biết một tên nhánh được rút gọn. Bạn có thể sử dụng `""` để ẩn biểu tượng. |
| `only_attached`      | `false`                          | Only show the branch name when not in a detached `HEAD` state.                                        |
| `disabled`           | `false`                          | Vô hiệu mô đun `git_branch`.                                                                          |

### Các biến

| Biến          | Ví dụ    | Mô tả                                                                                                  |
| ------------- | -------- | ------------------------------------------------------------------------------------------------------ |
| branch        | `master` | The current branch name, falls back to `HEAD` if there's no current branch (e.g. git detached `HEAD`). |
| remote_name   | `origin` | Tên remote.                                                                                            |
| remote_branch | `master` | Tên của nhánh đã theo dõi trên `remote_name`.                                                          |
| symbol        |          | Giá trị ghi đè tuỳ chọn `symbol`                                                                       |
| style\*     |          | Giá trị ghi đè của `style`                                                                             |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[git_branch]
symbol = "🌱 "
truncation_length = 4
truncation_symbol = ""
```

## Git Commit

Mô đun `git_commit` hiển thị hash commit hiện tại và tag (nếu có) của repo trong thư mục hiện tại của bạn.

### Các tuỳ chọn

| Tuỳ chọn             | Mặc định                                               | Mô tả                                                     |
| -------------------- | ------------------------------------------------------ | --------------------------------------------------------- |
| `commit_hash_length` | `7`                                                    | Độ dài của git commit hash được hiển thị.                 |
| `format`             | `"[\\($hash\\)]($style) [\\($tag\\)]($style)"` | Định dạng cho module.                                     |
| `style`              | `"bold green"`                                         | Kiểu cho module.                                          |
| `only_detached`      | `true`                                                 | Only show git commit hash when in detached `HEAD` state   |
| `tag_disabled`       | `true`                                                 | Vô hiệu hiển thị thông tin tag trong mô đun `git_commit`. |
| `tag_symbol`         | `" 🏷 "`                                                | Biểu tượng tag trước thông tin được hiển thị              |
| `disabled`           | `false`                                                | Vô hiệu mô đun `git_commit`.                              |

### Các biến

| Biến      | Ví dụ     | Mô tả                      |
| --------- | --------- | -------------------------- |
| hash      | `b703eb3` | Git commit hash hiện tại   |
| style\* |           | Giá trị ghi đè của `style` |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[git_commit]
commit_hash_length = 4
tag_symbol = "🔖 "
```

## Git State

Mô đun `git_state` sẽ hiển hiển thị trong các thư mục là một phần của gt repository và những nơi tồn tại một hoạt động trong tiến trình như _REBASING_, _BISECTING_. Nếu có thông tin tiến trình (ví dụ, REBASING 3/10), thông tin đó cũng sẽ được hiển thị.

### Các tuỳ chọn

| Tuỳ chọn       | Mặc định                                                        | Mô tả                                                                              |
| -------------- | --------------------------------------------------------------- | ---------------------------------------------------------------------------------- |
| `rebase`       | `"REBASING"`                                                    | Một format sring hiển thị khi một `rebase` đang trong quá trình.                   |
| `merge`        | `"MERGING"`                                                     | Một format sring hiển thị khi một `merge` đang trong quá trình.                    |
| `revert`       | `"REVERTING"`                                                   | Một format sring hiển thị khi một `revert` đang trong quá trình.                   |
| `cherry_pick`  | `"CHERRY-PICKING"`                                              | Một format sring hiển thị khi một `cherry-pick` đang trong quá trình.              |
| `bisect`       | `"BISECTING"`                                                   | Một format sring hiển thị khi một `bisect` đang trong quá trình.                   |
| `am`           | `"AM"`                                                          | Một format sring hiển thị khi một `apply-mailbox` (`git am`) đang trong quá trình. |
| `am_or_rebase` | `"AM/REBASE"`                                                   | Một format sring hiển thị khi một `apply-mailbox` (`rebase`) đang trong quá trình. |
| `style`        | `"bold yellow"`                                                 | Kiểu cho module.                                                                   |
| `format`       | `'\([$state( $progress_current/$progress_total)]($style)\) '` | Định dạng cho module.                                                              |
| `disabled`     | `false`                                                         | Vô hiệu `git_state` module.                                                        |

### Các biến

| Biến             | Ví dụ      | Mô tả                             |
| ---------------- | ---------- | --------------------------------- |
| state            | `REBASING` | Trạng thái của repo hiện tại      |
| progress_current | `1`        | Trạng thái của quá trình hiện tại |
| progress_total   | `2`        | Tổng số các quá trình             |
| style\*        |            | Giá trị ghi đè của `style`        |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[git_state]
format = '[\($state( $progress_current of $progress_total)\)]($style) '
cherry_pick = "[🍒 PICKING](bold red)"
```

## Git Status

Mô đun `git_status` hiển thị các biểu tượng đại diện cho trạng thái của repo trong thư mục hiện tại của bạn.

### Các tuỳ chọn

| Tuỳ chọn     | Mặc định                                        | Mô tả                               |
| ------------ | ----------------------------------------------- | ----------------------------------- |
| `format`     | `'([\[$all_status$ahead_behind\]]($style) )'` | Định dạng mặc định cho `git_status` |
| `conflicted` | `"="`                                           | Nhánh này có nhiều merge conflicts. |
| `ahead`      | `"⇡"`                                           | Định dạng của `ahead`               |
| `behind`     | `"⇣"`                                           | Định dạng của `behind`              |
| `diverged`   | `"⇕"`                                           | Định dạng của `diverged`            |
| `untracked`  | `"?"`                                           | Định dạng của `untracked`           |
| `stashed`    | `"$"`                                           | Định dạng của `stashed`             |
| `modified`   | `"!"`                                           | Định dạng của `modified`            |
| `staged`     | `"+"`                                           | Định dạng của `modified`            |
| `renamed`    | `"»"`                                           | Định dạng của `renamed`             |
| `deleted`    | `"✘"`                                           | Định dạng của `deleted`             |
| `style`      | `"bold red"`                                    | Kiểu cho module.                    |
| `disabled`   | `false`                                         | Vô hiệu `git_status` module.        |

### Các biến

Các biến dưới đây có thể được sử dụng trong `format`:

| Biến           | Mô tả                                                                                           |
| -------------- | ----------------------------------------------------------------------------------------------- |
| `all_status`   | Shortcut cho `$conflicted$stashed$deleted$renamed$modified$staged$untracked`                    |
| `ahead_behind` | Hiển thị format string của `diverged` `ahead` or `behind` dựa trên trạng thái hiện tại của repo |
| `conflicted`   | Hiển thị `conflicted` khi nhánh này có merge conflicts.                                         |
| `untracked`    | Hiển thị `untracked` khi có tệp tin untracked trong thư mục làm việc.                           |
| `stashed`      | Hiển thị `stashed` khi một stash tồn tại trong local repository.                                |
| `modified`     | Hiển thị `modified` khi có tệp tin được chỉnh sửa trong thư mục làm việc.                       |
| `staged`       | Hiển thị `staged` khi một tệp tin mới được thêm vào staging area.                               |
| `renamed`      | Hiển thị `renamed` khi một tệp tin đổi tên đã được thêm vào staging area.                       |
| `deleted`      | Hiển thị `deleted` khi một tệp tin bị xóa đã được thêm vào staging area.                        |
| style\*      | Giá trị ghi đè của `style`                                                                      |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

Các biến sau có thể được sử dụng trong `diverged`:

| Biến           | Mô tả                                         |
| -------------- | --------------------------------------------- |
| `ahead_count`  | Số lượng commit phía trước của nhánh tracking |
| `behind_count` | Số lượng commit phía sau nhánh tracking       |

Các biến sau có thể được sử dụng trong `conflicted`, `ahead`, `behind`, `untracked`, `stashed`, `modified`, `staged`, `renamed` and `deleted`:

| Biến    | Mô tả                         |
| ------- | ----------------------------- |
| `count` | Hiển thị số lượng các tệp tin |

### Ví dụ

```toml
# ~/.config/starship.toml

[git_status]
conflicted = "🏳"
ahead = "🏎💨"
behind = "😰"
diverged = "😵"
untracked = "🤷‍"
stashed = "📦"
modified = "📝"
staged = '[++\($count\)](green)'
renamed = "👅"
deleted = "🗑"
```

Hiển thị tổng số nhánh phía trước/phía sau của nhánh được track

```toml
# ~/.config/starship.toml

[git_status]
ahead = "⇡${count}"
diverged = "⇕⇡${ahead_count}⇣${behind_count}"
behind = "⇣${count}"
```

## Golang

The `golang` module shows the currently installed version of [Golang](https://golang.org/). Mặc định module sẽ được hiển thị nếu có bất kì điều kiện nào dưới đây thoả mãn:

- Thư mục hiện tại chứa một tập tin `go.mod`
- Đường dẫn hiện tại chứa một tập tin `go.sum`
- Thư mục hiện tại chứa một tập tin `glide.yaml`
- Thư mục hiện tại chứa một tập tin `Gopkg.yml`
- Đường dẫn hiện tại chứa một tập tin `Gopkg.lock`
- Thư mục hiện tại chứa một tệp tin `.go-version`
- Thư mục hiện tại chứa một thư mục `Godeps`
- Thư mục hiện tại chứa một tệp tin với phần mở rộng `.go`

### Các tuỳ chọn

| Tuỳ chọn            | Mặc định                                                                       | Mô tả                                                                     |
| ------------------- | ------------------------------------------------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`                                           | Định dạng cho module.                                                     |
| `version_format`    | `"v${raw}"`                                                                    | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🐹 "`                                                                         | Một format string đại diện cho biểu tượng của Go.                         |
| `detect_extensions` | `["go"]`                                                                       | Những tiện ích mở rộng nào sẽ kích hoạt mô-đun này.                       |
| `detect_files`      | `["go.mod", "go.sum", "glide.yaml", "Gopkg.yml", "Gopkg.lock", ".go-version"]` | Tên tệp nào sẽ kích hoạt mô-đun này.                                      |
| `detect_folders`    | `["Godeps"]`                                                                   | Những thư mục nào sẽ kích hoạt mô-đun này.                                |
| `style`             | `"bold cyan"`                                                                  | Kiểu cho module.                                                          |
| `disabled`          | `false`                                                                        | Vô hiệu `golang` module.                                                  |

### Các biến

| Biến      | Ví dụ     | Mô tả                            |
| --------- | --------- | -------------------------------- |
| version   | `v1.12.1` | Phiên bản của `go`               |
| symbol    |           | Giá trị ghi đè tuỳ chọn `symbol` |
| style\* |           | Giá trị ghi đè của `style`       |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[golang]
format = "via [🏎💨 $version](bold cyan) "
```

## Helm

The `helm` module shows the currently installed version of [Helm](https://helm.sh/). Mặc định module sẽ được hiển thị nếu có bất kì điều kiện nào dưới đây thoả mãn:

- Đường dẫn hiện tại chứa một tập tin `helmfile.yaml`
- Thư mục hiện tại chứa một tập tin `Chart.yaml`

### Các tuỳ chọn

| Tuỳ chọn            | Mặc định                             | Mô tả                                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Định dạng cho module.                                                     |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `[]`                                 | Những tiện ích mở rộng nào sẽ kích hoạt mô-đun này.                       |
| `detect_files`      | `["helmfile.yaml", "Chart.yaml"]`    | Tên tệp nào sẽ kích hoạt mô-đun này.                                      |
| `detect_folders`    | `[]`                                 | Những thư mục nào nên kích hoạt các mô đun này.                           |
| `symbol`            | `"⎈ "`                               | Một format string đại diện cho biểu tượng của Helm.                       |
| `style`             | `"bold white"`                       | Kiểu cho module.                                                          |
| `disabled`          | `false`                              | Vô hiệu `helm` module.                                                    |

### Các biến

| Biến      | Ví dụ    | Mô tả                            |
| --------- | -------- | -------------------------------- |
| version   | `v3.1.1` | Phiên bản của `helm`             |
| symbol    |          | Giá trị ghi đè tuỳ chọn `symbol` |
| style\* |          | Giá trị ghi đè của `style`       |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[helm]
format = "via [⎈ $version](bold white) "
```

## Hostname

Mô đun `hostname` hiển thị hostnam hệ thống.

### Các tuỳ chọn

| Tuỳ chọn   | Mặc định                    | Mô tả                                                                                                                            |
| ---------- | --------------------------- | -------------------------------------------------------------------------------------------------------------------------------- |
| `ssh_only` | `true`                      | Chỉ hiển thị hostname khi được kết nối tới một phiên SSH.                                                                        |
| `trim_at`  | `"."`                       | Chuỗi mà hostname được cắt ngắn, sau khi khớp lần đầu tiên. `"."` sẽ dừng sau dấu chấm đầu tiên. `""` sẽ vô hiệu mọi sự cắt ngắn |
| `format`   | `"[$hostname]($style) in "` | Định dạng cho module.                                                                                                            |
| `style`    | `"bold dimmed green"`       | Kiểu cho module.                                                                                                                 |
| `disabled` | `false`                     | Vô hiệu `hastname` module.                                                                                                       |

### Các biến

| Biến      | Ví dụ | Mô tả                            |
| --------- | ----- | -------------------------------- |
| symbol    |       | Giá trị ghi đè tuỳ chọn `symbol` |
| style\* |       | Giá trị ghi đè của `style`       |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[hostname]
ssh_only = false
format =  "on [$hostname](bold red) "
trim_at = ".companyname.com"
disabled = false
```

## Java

The `java` module shows the currently installed version of [Java](https://www.oracle.com/java/). Mặc định module sẽ được hiển thị nếu có bất kì điều kiện nào dưới đây thoả mãn:

- Thư mục hiện tại chứa một tệp tin `pom.xml`, `build.gradle.kts`, `build.sbt`, `.java-version`, `.deps.edn`, `project.clj`, or `build.boot`
- Thư mục hiện tại chứa một tệp tin với phần mở rộng `.java`, `.class`, `.gradle`, `.jar`, `.clj`, or `.cljc`

### Các tuỳ chọn

| Tuỳ chọn            | Mặc định                                                                                                  | Mô tả                                                                     |
| ------------------- | --------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `"via [${symbol}(${version} )]($style)"`                                                                  | Định dạng cho module.                                                     |
| `version_format`    | `"v${raw}"`                                                                                               | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `["java", "class", "gradle", "jar", "cljs", "cljc"]`                                                      | Những tiện ích mở rộng nào sẽ kích hoạt mô-đun này.                       |
| `detect_files`      | `["pom.xml", "build.gradle.kts", "build.sbt", ".java-version", ".deps.edn", "project.clj", "build.boot"]` | Tên tệp nào sẽ kích hoạt mô-đun này.                                      |
| `detect_folders`    | `[]`                                                                                                      | Những thư mục nào nên kích hoạt các mô đun này.                           |
| `symbol`            | `"☕ "`                                                                                                    | Một format string đại diện cho biểu tượng Java                            |
| `style`             | `"red dimmed"`                                                                                            | Kiểu cho module.                                                          |
| `disabled`          | `false`                                                                                                   | Vô hiệu `java` module.                                                    |

### Các biến

| Biến      | Ví dụ | Mô tả                            |
| --------- | ----- | -------------------------------- |
| version   | `v14` | Phiên bản của `java`             |
| symbol    |       | Giá trị ghi đè tuỳ chọn `symbol` |
| style\* |       | Giá trị ghi đè của `style`       |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[java]
symbol = "🌟 "
```

## Jobs

`jobs` module cho biết số lượng các jobs đang chạy. Mô đun sẽ được hiển thị chỉ khi có background jobs đang chạy. Mô đúnẽ hiển thị số lượng jobs đang chỵ nếu có nhiều hơn 1 jobs, hoặc nhiều hơn giá trị cấu hình `threshold`, nếu nó tồn tại. If `threshold` is set to 0, then the module will also show when there are 0 jobs running.

::: cảnh báo

Mô đun này không hỗ trợ trên tcsh.

:::

### Các tuỳ chọn

| Tuỳ chọn    | Mặc định                      | Mô tả                                        |
| ----------- | ----------------------------- | -------------------------------------------- |
| `threshold` | `1`                           | Cho biết số lượng jobs nếu nó vượt quá.      |
| `format`    | `"[$symbol$number]($style) "` | Định dạng cho module.                        |
| `symbol`    | `"✦"`                         | Một format string đại diện cho số lượng job. |
| `style`     | `"bold blue"`                 | Kiểu cho module.                             |
| `disabled`  | `false`                       | Vô hiệu `jobs` module.                       |

### Các biến

| Biến      | Ví dụ | Mô tả                            |
| --------- | ----- | -------------------------------- |
| number    | `1`   | Số lượng job                     |
| symbol    |       | Giá trị ghi đè tuỳ chọn `symbol` |
| style\* |       | Giá trị ghi đè của `style`       |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[jobs]
symbol = "+ "
threshold = 4
```

## Julia

The `julia` module shows the currently installed version of [Julia](https://julialang.org/). Mặc định module sẽ được hiển thị nếu có bất kì điều kiện nào dưới đây thoả mãn:

- Thư mục hiện tại chứa một tệp tin `Project.toml`
- Thư mục hiện tại chứa một tập tin `Manifest.toml`
- Thư mục hiện tại chứa một tệp tin với phần mở rộng `.jl`

### Các tuỳ chọn

| Tuỳ chọn            | Mặc định                             | Mô tả                                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Định dạng cho module.                                                     |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `["jl"]`                             | Những tiện ích mở rộng nào sẽ kích hoạt mô-đun này.                       |
| `detect_files`      | `["Project.toml", "Manifest.toml"]`  | Tên tệp nào sẽ kích hoạt mô-đun này.                                      |
| `detect_folders`    | `[]`                                 | Những thư mục nào nên kích hoạt các mô đun này.                           |
| `symbol`            | `"ஃ "`                               | Một format string đại diện cho biếu tượng của Julia.                      |
| `style`             | `"bold purple"`                      | Kiểu cho module.                                                          |
| `disabled`          | `false`                              | Vô hiệu `julia` module.                                                   |

### Các biến

| Biến      | Ví dụ    | Mô tả                            |
| --------- | -------- | -------------------------------- |
| version   | `v1.4.0` | Phiên bản của `julia`            |
| symbol    |          | Giá trị ghi đè tuỳ chọn `symbol` |
| style\* |          | Giá trị ghi đè của `style`       |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[julia]
symbol = "∴ "
```

## Kotlin

The `kotlin` module shows the currently installed version of [Kotlin](https://kotlinlang.org/). Mặc định module sẽ được hiển thị nếu có bất kì điều kiện nào dưới đây thoả mãn:

- Thư mục hiện tại chứa một tệp tin `.kt` hoặc một tệp tin `.kts`

### Các tuỳ chọn

| Tuỳ chọn            | Mặc định                             | Mô tả                                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Định dạng cho module.                                                     |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `["kt", "kts"]`                      | Những tiện ích mở rộng nào sẽ kích hoạt mô-đun này.                       |
| `detect_files`      | `[]`                                 | Tên tệp nào sẽ kích hoạt mô-đun này.                                      |
| `detect_folders`    | `[]`                                 | Những thư mục nào nên kích hoạt các mô đun này.                           |
| `symbol`            | `"🅺 "`                               | Một format string đại diện cho biết tượng của Kotllin.                    |
| `style`             | `"bold blue"`                        | Kiểu cho module.                                                          |
| `kotlin_binary`     | `"kotlin"`                           | Cấu hình kotlin nhị phân mà Starship thực thi khi lấy phiên bản.          |
| `disabled`          | `false`                              | Vô hiệu `kotlin` module.                                                  |

### Các biến

| Biến      | Ví dụ     | Mô tả                            |
| --------- | --------- | -------------------------------- |
| version   | `v1.4.21` | Phiên bản của `kotlin`           |
| symbol    |           | Giá trị ghi đè tuỳ chọn `symbol` |
| style\* |           | Giá trị ghi đè của `style`       |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[kotlin]
symbol = "🅺 "
```

```toml
# ~/.config/starship.toml

[kotlin]
# Sử dụng Kitlin Compiler nhị phân để lấy phiên bản được cài đặt
kotlin_binary = "kotlinc"
```

## Kubernetes

Displays the current [Kubernetes context](https://kubernetes.io/docs/concepts/configuration/organize-cluster-access-kubeconfig/#context) name and, if set, the namespace from the kubeconfig file. Namespace cần được thiết lập trong tệp tin kubeconfig, cài này có thể được thực thi thông qua `kubectl config set-context starship-cluster --namespace astronaut`. Nếu biến môi trường `$KUBECONFIG` được thiết lập, mô đun sẽ sử dụng cái đó nếu nó không sử dụng `~/.kube/config`.

::: thử thuật

Mặc định, mô đun này được vô hiệu. Để kích hoạt nó, thiết lập `disabled` sang `false` trong tập tin cấu hình của bạn.

:::

### Các tuỳ chọn

| Tuỳ chọn          | Mặc định                                             | Mô tả                                                                 |
| ----------------- | ---------------------------------------------------- | --------------------------------------------------------------------- |
| `symbol`          | `"☸ "`                                               | A format string representing the symbol displayed before the Cluster. |
| `format`          | `'[$symbol$context( \($namespace\))]($style) in '` | Định dạng cho module.                                                 |
| `style`           | `"cyan bold"`                                        | Kiểu cho module.                                                      |
| `context_aliases` |                                                      | Table of context aliases to display.                                  |
| `disabled`        | `true`                                               | Disables the `kubernetes` module.                                     |

### Các biến

| Biến      | Ví dụ                | Mô tả                                    |
| --------- | -------------------- | ---------------------------------------- |
| context   | `starship-cluster`   | The current kubernetes context           |
| namespace | `starship-namespace` | If set, the current kubernetes namespace |
| symbol    |                      | Giá trị ghi đè tuỳ chọn `symbol`         |
| style\* |                      | Giá trị ghi đè của `style`               |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[kubernetes]
format = 'on [⛵ $context \($namespace\)](dimmed green) '
disabled = false
[kubernetes.context_aliases]
"dev.local.cluster.k8s" = "dev"
```

## Line Break

The `line_break` module separates the prompt into two lines.

### Các tuỳ chọn

| Tuỳ chọn   | Mặc định | Mô tả                                                              |
| ---------- | -------- | ------------------------------------------------------------------ |
| `disabled` | `false`  | Disables the `line_break` module, making the prompt a single line. |

### Ví dụ

```toml
# ~/.config/starship.toml

[line_break]
disabled = true
```

## Lua

The `lua` module shows the currently installed version of [Lua](http://www.lua.org/). Mặc định module sẽ được hiển thị nếu có bất kì điều kiện nào dưới đây thoả mãn:

- The current directory contains a `.lua-version` file
- The current directory contains a `lua` directory
- The current directory contains a file with the `.lua` extension

### Các tuỳ chọn

| Tuỳ chọn            | Mặc định                             | Mô tả                                                                      |
| ------------------- | ------------------------------------ | -------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Định dạng cho module.                                                      |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch`  |
| `symbol`            | `"🌙 "`                               | A format string representing the symbol of Lua.                            |
| `detect_extensions` | `["lua"]`                            | Những tiện ích mở rộng nào sẽ kích hoạt mô-đun này.                        |
| `detect_files`      | `[".lua-version"]`                   | Tên tệp nào sẽ kích hoạt mô-đun này.                                       |
| `detect_folders`    | `["lua"]`                            | Những thư mục nào sẽ kích hoạt mô-đun này.                                 |
| `style`             | `"bold blue"`                        | Kiểu cho module.                                                           |
| `lua_binary`        | `"lua"`                              | Configures the lua binary that Starship executes when getting the version. |
| `disabled`          | `false`                              | Disables the `lua` module.                                                 |

### Các biến

| Biến      | Ví dụ    | Mô tả                            |
| --------- | -------- | -------------------------------- |
| version   | `v5.4.0` | The version of `lua`             |
| symbol    |          | Giá trị ghi đè tuỳ chọn `symbol` |
| style\* |          | Giá trị ghi đè của `style`       |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[lua]
format = "via [🌕 $version](bold blue) "
```

## Memory Usage

The `memory_usage` module shows current system memory and swap usage.

By default the swap usage is displayed if the total system swap is non-zero.

::: thử thuật

Mặc định, mô đun này được vô hiệu. Để kích hoạt nó, thiết lập `disabled` sang `false` trong tập tin cấu hình của bạn.

:::

### Các tuỳ chọn

| Tuỳ chọn    | Mặc định                                        | Mô tả                                                    |
| ----------- | ----------------------------------------------- | -------------------------------------------------------- |
| `threshold` | `75`                                            | Hide the memory usage unless it exceeds this percentage. |
| `format`    | `"via $symbol [${ram}( \| ${swap})]($style) "` | Định dạng cho module.                                    |
| `symbol`    | `"🐏"`                                           | The symbol used before displaying the memory usage.      |
| `style`     | `"bold dimmed white"`                           | Kiểu cho module.                                         |
| `disabled`  | `true`                                          | Disables the `memory_usage` module.                      |

### Các biến

| Biến             | Ví dụ         | Mô tả                                                              |
| ---------------- | ------------- | ------------------------------------------------------------------ |
| ram              | `31GiB/65GiB` | The usage/total RAM of the current system memory.                  |
| ram_pct          | `48%`         | The percentage of the current system memory.                       |
| swap\*\*     | `1GiB/4GiB`   | The swap memory size of the current system swap memory file.       |
| swap_pct\*\* | `77%`         | The swap memory percentage of the current system swap memory file. |
| symbol           | `🐏`           | Giá trị ghi đè tuỳ chọn `symbol`                                   |
| style\*        |               | Giá trị ghi đè của `style`                                         |

\*: This variable can only be used as a part of a style string \*\*: The SWAP file information is only displayed if detected on the current system

### Ví dụ

```toml
# ~/.config/starship.toml

[memory_usage]
disabled = false
threshold = -1
symbol = " "
style = "bold dimmed green"
```

## Mercurial Branch

The `hg_branch` module shows the active branch of the repo in your current directory.

### Các tuỳ chọn

| Tuỳ chọn            | Mặc định                         | Mô tả                                                                                        |
| ------------------- | -------------------------------- | -------------------------------------------------------------------------------------------- |
| `symbol`            | `" "`                           | The symbol used before the hg bookmark or branch name of the repo in your current directory. |
| `style`             | `"bold purple"`                  | Kiểu cho module.                                                                             |
| `format`            | `"on [$symbol$branch]($style) "` | Định dạng cho module.                                                                        |
| `truncation_length` | `2^63 - 1`                       | Truncates the hg branch name to `N` graphemes                                                |
| `truncation_symbol` | `"…"`                            | Biểu tượng sử dụng để nhận biết một tên nhánh được rút gọn.                                  |
| `disabled`          | `true`                           | Disables the `hg_branch` module.                                                             |

### Các biến

| Biến      | Ví dụ    | Mô tả                            |
| --------- | -------- | -------------------------------- |
| branch    | `master` | The active mercurial branch      |
| symbol    |          | Giá trị ghi đè tuỳ chọn `symbol` |
| style\* |          | Giá trị ghi đè của `style`       |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[hg_branch]
format = "on [🌱 $branch](bold purple)"
truncation_length = 4
truncation_symbol = ""
```

## Nim

The `nim` module shows the currently installed version of [Nim](https://nim-lang.org/). Mặc định module sẽ được hiển thị nếu có bất kì điều kiện nào dưới đây thoả mãn:

- Đường dẫn hiện tại chứa một tập tin `nim.cfg`
- The current directory contains a file with the `.nim` extension
- The current directory contains a file with the `.nims` extension
- The current directory contains a file with the `.nimble` extension

### Các tuỳ chọn

| Tuỳ chọn            | Mặc định                             | Mô tả                                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Định dạng cho module                                                      |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"👑 "`                               | The symbol used before displaying the version of Nim.                     |
| `detect_extensions` | `["nim", "nims", "nimble"]`          | Những tiện ích mở rộng nào sẽ kích hoạt mô-đun này.                       |
| `detect_files`      | `["nim.cfg"]`                        | Tên tệp nào sẽ kích hoạt mô-đun này.                                      |
| `detect_folders`    | `[]`                                 | Những thư mục nào sẽ kích hoạt mô-đun này.                                |
| `style`             | `"bold yellow"`                      | Kiểu cho module.                                                          |
| `disabled`          | `false`                              | Disables the `nim` module.                                                |

### Các biến

| Biến      | Ví dụ    | Mô tả                            |
| --------- | -------- | -------------------------------- |
| version   | `v1.2.0` | The version of `nimc`            |
| symbol    |          | Giá trị ghi đè tuỳ chọn `symbol` |
| style\* |          | Giá trị ghi đè của `style`       |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[nim]
style = "yellow"
symbol = "🎣 "
```

## Nix-shell

The `nix_shell` module shows the [nix-shell](https://nixos.org/guides/nix-pills/developing-with-nix-shell.html) environment. The module will be shown when inside a nix-shell environment.

### Các tuỳ chọn

| Tuỳ chọn     | Mặc định                                       | Mô tả                                                 |
| ------------ | ---------------------------------------------- | ----------------------------------------------------- |
| `format`     | `'via [$symbol$state( \($name\))]($style) '` | Định dạng cho module.                                 |
| `symbol`     | `"❄️ "`                                        | A format string representing the symbol of nix-shell. |
| `style`      | `"bold blue"`                                  | Kiểu cho module.                                      |
| `impure_msg` | `"impure"`                                     | A format string shown when the shell is impure.       |
| `pure_msg`   | `"pure"`                                       | A format string shown when the shell is pure.         |
| `disabled`   | `false`                                        | Disables the `nix_shell` module.                      |

### Các biến

| Biến      | Ví dụ   | Mô tả                            |
| --------- | ------- | -------------------------------- |
| state     | `pure`  | The state of the nix-shell       |
| name      | `lorri` | The name of the nix-shell        |
| symbol    |         | Giá trị ghi đè tuỳ chọn `symbol` |
| style\* |         | Giá trị ghi đè của `style`       |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[nix_shell]
disabled = true
impure_msg = "[impure shell](bold red)"
pure_msg = "[pure shell](bold green)"
format = 'via [☃️ $state( \($name\))](bold blue) '
```

## Node.js

The `nodejs` module shows the currently installed version of [Node.js](https://nodejs.org/). Mặc định module sẽ được hiển thị nếu có bất kì điều kiện nào dưới đây thoả mãn:

- Đường dẫn hiện tại chứa một tập tin `package.json`
- The current directory contains a `.node-version` file
- The current directory contains a `.nvmrc` file
- The current directory contains a `node_modules` directory
- The current directory contains a file with the `.js`, `.mjs` or `.cjs` extension
- The current directory contains a file with the `.ts` extension

### Các tuỳ chọn

| Tuỳ chọn            | Mặc định                             | Mô tả                                                                                                 |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Định dạng cho module.                                                                                 |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch`                             |
| `symbol`            | `" "`                               | A format string representing the symbol of Node.js.                                                   |
| `detect_extensions` | `["js", "mjs", "cjs", "ts"]`         | Những tiện ích mở rộng nào sẽ kích hoạt mô-đun này.                                                   |
| `detect_files`      | `["package.json", ".node-version"]`  | Tên tệp nào sẽ kích hoạt mô-đun này.                                                                  |
| `detect_folders`    | `["node_modules"]`                   | Những thư mục nào sẽ kích hoạt mô-đun này.                                                            |
| `style`             | `"bold green"`                       | Kiểu cho module.                                                                                      |
| `disabled`          | `false`                              | Disables the `nodejs` module.                                                                         |
| `not_capable_style` | `bold red`                           | The style for the module when an engines property in package.json does not match the Node.js version. |

### Các biến

| Biến      | Ví dụ      | Mô tả                            |
| --------- | ---------- | -------------------------------- |
| version   | `v13.12.0` | The version of `node`            |
| symbol    |            | Giá trị ghi đè tuỳ chọn `symbol` |
| style\* |            | Giá trị ghi đè của `style`       |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[nodejs]
format = "via [🤖 $version](bold green) "
```

## OCaml

The `ocaml` module shows the currently installed version of [OCaml](https://ocaml.org/). Mặc định module sẽ được hiển thị nếu có bất kì điều kiện nào dưới đây thoả mãn:

- The current directory contains a file with `.opam` extension or `_opam` directory
- The current directory contains a `esy.lock` directory
- The current directory contains a `dune` or `dune-project` file
- The current directory contains a `jbuild` or `jbuild-ignore` file
- The current directory contains a `.merlin` file
- The current directory contains a file with `.ml`, `.mli`, `.re` or `.rei` extension

### Các tuỳ chọn

| Tuỳ chọn                  | Mặc định                                                                   | Mô tả                                                                     |
| ------------------------- | -------------------------------------------------------------------------- | ------------------------------------------------------------------------- |
| `format`                  | `"via [$symbol($version )(\($switch_indicator$switch_name\) )]($style)"` | The format string for the module.                                         |
| `version_format`          | `"v${raw}"`                                                                | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`                  | `"🐫 "`                                                                     | The symbol used before displaying the version of OCaml.                   |
| `global_switch_indicator` | `""`                                                                       | The format string used to represent global OPAM switch.                   |
| `local_switch_indicator`  | `"*"`                                                                      | The format string used to represent local OPAM switch.                    |
| `detect_extensions`       | `["opam", "ml", "mli", "re", "rei"]`                                       | Những tiện ích mở rộng nào sẽ kích hoạt mô-đun này.                       |
| `detect_files`            | `["dune", "dune-project", "jbuild", "jbuild-ignore", ".merlin"]`           | Tên tệp nào sẽ kích hoạt mô-đun này.                                      |
| `detect_folders`          | `["_opam", "esy.lock"]`                                                    | Những thư mục nào sẽ kích hoạt mô-đun này.                                |
| `style`                   | `"bold yellow"`                                                            | Kiểu cho module.                                                          |
| `disabled`                | `false`                                                                    | Disables the `ocaml` module.                                              |

### Các biến

| Biến             | Ví dụ        | Mô tả                                                             |
| ---------------- | ------------ | ----------------------------------------------------------------- |
| version          | `v4.10.0`    | The version of `ocaml`                                            |
| switch_name      | `my-project` | The active OPAM switch                                            |
| switch_indicator |              | Mirrors the value of `indicator` for currently active OPAM switch |
| symbol           |              | Giá trị ghi đè tuỳ chọn `symbol`                                  |
| style\*        |              | Giá trị ghi đè của `style`                                        |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[ocaml]
format = "via [🐪 $version]($style) "
```

## OpenStack

The `openstack` module shows the current OpenStack cloud and project. The module only active when the `OS_CLOUD` env var is set, in which case it will read `clouds.yaml` file from any of the [default locations](https://docs.openstack.org/python-openstackclient/latest/configuration/index.html#configuration-files). to fetch the current project in use.

### Các tuỳ chọn

| Tuỳ chọn   | Mặc định                                            | Mô tả                                                          |
| ---------- | --------------------------------------------------- | -------------------------------------------------------------- |
| `format`   | `"on [$symbol$cloud(\\($project\\))]($style) "` | Định dạng cho module.                                          |
| `symbol`   | `"☁️ "`                                             | The symbol used before displaying the current OpenStack cloud. |
| `style`    | `"bold yellow"`                                     | Kiểu cho module.                                               |
| `disabled` | `false`                                             | Disables the `openstack` module.                               |

### Các biến

| Biến      | Ví dụ  | Mô tả                            |
| --------- | ------ | -------------------------------- |
| cloud     | `corp` | The current OpenStack cloud      |
| project   | `dev`  | The current OpenStack project    |
| symbol    |        | Giá trị ghi đè tuỳ chọn `symbol` |
| style\* |        | Giá trị ghi đè của `style`       |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[openstack]
format = "on [$symbol$cloud(\\($project\\))]($style) "
style = "bold yellow"
symbol = "☁️ "
```

## Package Version

The `package` module is shown when the current directory is the repository for a package, and shows its current version. The module currently supports `npm`, `nimble`, `cargo`, `poetry`, `composer`, `gradle`, `julia`, `mix` and `helm` packages.

- [**npm**](https://docs.npmjs.com/cli/commands/npm) – The `npm` package version is extracted from the `package.json` present in the current directory
- [**cargo**](https://doc.rust-lang.org/cargo/) – The `cargo` package version is extracted from the `Cargo.toml` present in the current directory
- [**nimble**](https://github.com/nim-lang/nimble) - The `nimble` package version is extracted from the `*.nimble` file present in the current directory with the `nimble dump` command
- [**poetry**](https://python-poetry.org/) – The `poetry` package version is extracted from the `pyproject.toml` present in the current directory
- [**composer**](https://getcomposer.org/) – The `composer` package version is extracted from the `composer.json` present in the current directory
- [**gradle**](https://gradle.org/) – The `gradle` package version is extracted from the `build.gradle` present
- [**julia**](https://docs.julialang.org/en/v1/stdlib/Pkg/) - The package version is extracted from the `Project.toml` present
- [**mix**](https://hexdocs.pm/mix/) - The `mix` package version is extracted from the `mix.exs` present
- [**helm**](https://helm.sh/docs/helm/helm_package/) - The `helm` chart version is extracted from the `Chart.yaml` present
- [**maven**](https://maven.apache.org/) - The `maven` package version is extracted from the `pom.xml` present
- [**meson**](https://mesonbuild.com/) - The `meson` package version is extracted from the `meson.build` present
- [**vlang**](https://vlang.io) - The `vlang` package version is extracted from the `v.mod` present

> ⚠️ The version being shown is that of the package whose source code is in your current directory, not your package manager.

### Các tuỳ chọn

| Tuỳ chọn          | Mặc định                          | Mô tả                                                      |
| ----------------- | --------------------------------- | ---------------------------------------------------------- |
| `format`          | `"is [$symbol$version]($style) "` | Định dạng cho module.                                      |
| `symbol`          | `"📦 "`                            | The symbol used before displaying the version the package. |
| `style`           | `"bold 208"`                      | Kiểu cho module.                                           |
| `display_private` | `false`                           | Enable displaying version for packages marked as private.  |
| `disabled`        | `false`                           | Disables the `package` module.                             |

### Các biến

| Biến      | Ví dụ    | Mô tả                            |
| --------- | -------- | -------------------------------- |
| version   | `v1.0.0` | The version of your package      |
| symbol    |          | Giá trị ghi đè tuỳ chọn `symbol` |
| style\* |          | Giá trị ghi đè của `style`       |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[package]
format = "via [🎁 $version](208 bold) "
```

## Perl

The `perl` module shows the currently installed version of [Perl](https://www.perl.org/). Mặc định module sẽ được hiển thị nếu có bất kì điều kiện nào dưới đây thoả mãn:

- The current directory contains a `Makefile.PL` or `Build.PL` file
- The current directory contains a `cpanfile` or `cpanfile.snapshot` file
- The current directory contains a `META.json` file or `META.yml` file
- The current directory contains a `.perl-version` file
- The current directory contains a `.pl`, `.pm` or `.pod`

### Các tuỳ chọn

| Tuỳ chọn            | Mặc định                                                                                                 | Mô tả                                                                     |
| ------------------- | -------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`                                                                     | The format string for the module.                                         |
| `version_format`    | `"v${raw}"`                                                                                              | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🐪 "`                                                                                                   | The symbol used before displaying the version of Perl                     |
| `detect_extensions` | `["pl", "pm", "pod"]`                                                                                    | Những tiện ích mở rộng nào sẽ kích hoạt mô-đun này.                       |
| `detect_files`      | `["Makefile.PL", "Build.PL", "cpanfile", "cpanfile.snapshot", "META.json", "META.yml", ".perl-version"]` | Tên tệp nào sẽ kích hoạt mô-đun này.                                      |
| `detect_folders`    | `[]`                                                                                                     | Những thư mục nào sẽ kích hoạt mô-đun này.                                |
| `style`             | `"bold 149"`                                                                                             | Kiểu cho module.                                                          |
| `disabled`          | `false`                                                                                                  | Disables the `perl` module.                                               |

### Các biến

| Biến      | Ví dụ     | Mô tả                            |
| --------- | --------- | -------------------------------- |
| version   | `v5.26.1` | The version of `perl`            |
| symbol    |           | Giá trị ghi đè tuỳ chọn `symbol` |
| style\* |           | Giá trị ghi đè của `style`       |

### Ví dụ

```toml
# ~/.config/starship.toml

[perl]
format = "via [🦪 $version]($style) "
```

## PHP

The `php` module shows the currently installed version of [PHP](https://www.php.net/). Mặc định module sẽ được hiển thị nếu có bất kì điều kiện nào dưới đây thoả mãn:

- Đường dẫn hiện tại chứa một tập tin `composer.json`
- The current directory contains a `.php-version` file
- The current directory contains a `.php` extension

### Các tuỳ chọn

| Tuỳ chọn            | Mặc định                             | Mô tả                                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Định dạng cho module.                                                     |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🐘 "`                               | The symbol used before displaying the version of PHP.                     |
| `detect_extensions` | `["php"]`                            | Những tiện ích mở rộng nào sẽ kích hoạt mô-đun này.                       |
| `detect_files`      | `["composer.json", ".php-version"]`  | Tên tệp nào sẽ kích hoạt mô-đun này.                                      |
| `detect_folders`    | `[]`                                 | Những thư mục nào sẽ kích hoạt mô-đun này.                                |
| `style`             | `"147 bold"`                         | Kiểu cho module.                                                          |
| `disabled`          | `false`                              | Disables the `php` module.                                                |

### Các biến

| Biến      | Ví dụ    | Mô tả                            |
| --------- | -------- | -------------------------------- |
| version   | `v7.3.8` | The version of `php`             |
| symbol    |          | Giá trị ghi đè tuỳ chọn `symbol` |
| style\* |          | Giá trị ghi đè của `style`       |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[php]
format = "via [🔹 $version](147 bold) "
```

## PureScript

The `purescript` module shows the currently installed version of [PureScript](https://www.purescript.org/) version. Mặc định module sẽ được hiển thị nếu có bất kì điều kiện nào dưới đây thoả mãn:

- Đường dẫn hiện tại chứa một tập tin `spago.dhall`
- The current directory contains a file with the `.purs` extension

### Các tuỳ chọn

| Tuỳ chọn            | Mặc định                             | Mô tả                                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Định dạng cho module.                                                     |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"<=> "`                       | The symbol used before displaying the version of PureScript.              |
| `detect_extensions` | `["purs"]`                           | Những tiện ích mở rộng nào sẽ kích hoạt mô-đun này.                       |
| `detect_files`      | `["spago.dhall"]`                    | Tên tệp nào sẽ kích hoạt mô-đun này.                                      |
| `detect_folders`    | `[]`                                 | Những thư mục nào sẽ kích hoạt mô-đun này.                                |
| `style`             | `"bold white"`                       | Kiểu cho module.                                                          |
| `disabled`          | `false`                              | Disables the `purescript` module.                                         |

### Các biến

| Biến      | Ví dụ    | Mô tả                            |
| --------- | -------- | -------------------------------- |
| version   | `0.13.5` | The version of `purescript`      |
| symbol    |          | Giá trị ghi đè tuỳ chọn `symbol` |
| style\* |          | Giá trị ghi đè của `style`       |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[purescript]
format = "via [$symbol$version](bold white)"
```

## Python

The `python` module shows the currently installed version of [Python](https://www.python.org/) and the current [Python virtual environment](https://docs.python.org/tutorial/venv.html) if one is activated.

If `pyenv_version_name` is set to `true`, it will display the pyenv version name. Otherwise, it will display the version number from `python --version`.

Mặc định module sẽ được hiển thị nếu có bất kì điều kiện nào dưới đây thoả mãn:

- The current directory contains a `.python-version` file
- The current directory contains a `Pipfile` file
- The current directory contains a `__init__.py` file
- Đường dẫn hiện tại chứa một tập tin `pyproject.toml`
- Đường dẫn hiện tại chứa một tập tin `requirements.txt`
- Đường dẫn hiện tại chứa một tập tin `setup.py`
- Đường dẫn hiện tại chứa một tập tin `tox.ini`
- The current directory contains a file with the `.py` extension.
- A virtual environment is currently activated

### Các tuỳ chọn

| Tuỳ chọn             | Mặc định                                                                                                     | Mô tả                                                                                  |
| -------------------- | ------------------------------------------------------------------------------------------------------------ | -------------------------------------------------------------------------------------- |
| `format`             | `'via [${symbol}${pyenv_prefix}(${version} )(\($virtualenv\) )]($style)'`                                  | Định dạng cho module.                                                                  |
| `version_format`     | `"v${raw}"`                                                                                                  | The version format. Available vars are `raw`, `major`, `minor`, & `patch`              |
| `symbol`             | `"🐍 "`                                                                                                       | A format string representing the symbol of Python                                      |
| `style`              | `"yellow bold"`                                                                                              | Kiểu cho module.                                                                       |
| `pyenv_version_name` | `false`                                                                                                      | Use pyenv to get Python version                                                        |
| `pyenv_prefix`       | `pyenv`                                                                                                      | Prefix before pyenv version display, only used if pyenv is used                        |
| `python_binary`      | `["python", "python3, "python2"]`                                                                            | Configures the python binaries that Starship should executes when getting the version. |
| `detect_extensions`  | `["py"]`                                                                                                     | Những tiện ích mở rộng nào sẽ kích hoạt mô-đun này                                     |
| `detect_files`       | `[".python-version", "Pipfile", "__init__.py", "pyproject.toml", "requirements.txt", "setup.py", "tox.ini"]` | Tên tệp nào sẽ kích hoạt mô-đun này                                                    |
| `detect_folders`     | `[]`                                                                                                         | Thư mục nào sẽ kích hoạt mô-đun này                                                    |
| `disabled`           | `false`                                                                                                      | Disables the `python` module.                                                          |

::: thử thuật

The `python_binary` variable accepts either a string or a list of strings. Starship will try executing each binary until it gets a result. Note you can only change the binary that Starship executes to get the version of Python not the arguments that are used.

The default values and order for `python_binary` was chosen to first identify the Python version in a virtualenv/conda environments (which currently still add a `python`, no matter if it points to `python3` or `python2`). This has the side effect that if you still have a system Python 2 installed, it may be picked up before any Python 3 (at least on Linux Distros that always symlink `/usr/bin/python` to Python 2). If you do not work with Python 2 anymore but cannot remove the system Python 2, changing this to `"python3"` will hide any Python version 2, see example below.

:::

### Các biến

| Biến         | Ví dụ           | Mô tả                                      |
| ------------ | --------------- | ------------------------------------------ |
| version      | `"v3.8.1"`      | The version of `python`                    |
| symbol       | `"🐍 "`          | Giá trị ghi đè tuỳ chọn `symbol`           |
| style        | `"yellow bold"` | Giá trị ghi đè của `style`                 |
| pyenv_prefix | `"pyenv "`      | Mirrors the value of option `pyenv_prefix` |
| virtualenv   | `"venv"`        | The current `virtualenv` name              |

### Ví dụ

```toml
# ~/.config/starship.toml

[python]
symbol = "👾 "
pyenv_version_name = true
```

```toml
# ~/.config/starship.toml

[python]
# Only use the `python3` binary to get the version.
python_binary = "python3"
```

```toml
# ~/.config/starship.toml

[python]
# Don't trigger for files with the py extension
detect_extensions = []
```

## Red

By default the `red` module shows the currently installed version of [Red](https://www.red-lang.org/). Module cho sẽ được hiện nếu bất kì điều kiện nào dưới đây thoả mãn:

- The current directory contains a file with `.red` or `.reds` extension

### Các tuỳ chọn

| Tuỳ chọn            | Mặc định                             | Mô tả                                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Định dạng cho module.                                                     |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🔺 "`                               | A format string representing the symbol of Red.                           |
| `detect_extensions` | `["red"]`                            | Những tiện ích mở rộng nào sẽ kích hoạt mô-đun này.                       |
| `detect_files`      | `[]`                                 | Tên tệp nào sẽ kích hoạt mô-đun này.                                      |
| `detect_folders`    | `[]`                                 | Những thư mục nào sẽ kích hoạt mô-đun này.                                |
| `style`             | `"red bold"`                         | Kiểu cho module.                                                          |
| `disabled`          | `false`                              | Disables the `red` module.                                                |

### Các biến

| Biến      | Ví dụ    | Mô tả                            |
| --------- | -------- | -------------------------------- |
| version   | `v2.5.1` | The version of `red`             |
| symbol    |          | Giá trị ghi đè tuỳ chọn `symbol` |
| style\* |          | Giá trị ghi đè của `style`       |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[red]
symbol = "🔴 "
```

## Ruby

By default the `ruby` module shows the currently installed version of [Ruby](https://www.ruby-lang.org/). Module cho sẽ được hiện nếu bất kì điều kiện nào dưới đây thoả mãn:

- The current directory contains a `Gemfile` file
- The current directory contains a `.ruby-version` file
- The current directory contains a `.rb` file

### Các tuỳ chọn

| Tuỳ chọn            | Mặc định                             | Mô tả                                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Định dạng cho module.                                                     |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"💎 "`                               | A format string representing the symbol of Ruby.                          |
| `detect_extensions` | `["rb"]`                             | Những tiện ích mở rộng nào sẽ kích hoạt mô-đun này.                       |
| `detect_files`      | `["Gemfile", ".ruby-version"]`       | Tên tệp nào sẽ kích hoạt mô-đun này.                                      |
| `detect_folders`    | `[]`                                 | Những thư mục nào sẽ kích hoạt mô-đun này.                                |
| `style`             | `"bold red"`                         | Kiểu cho module.                                                          |
| `disabled`          | `false`                              | Disables the `ruby` module.                                               |

### Các biến

| Biến      | Ví dụ    | Mô tả                            |
| --------- | -------- | -------------------------------- |
| version   | `v2.5.1` | The version of `ruby`            |
| symbol    |          | Giá trị ghi đè tuỳ chọn `symbol` |
| style\* |          | Giá trị ghi đè của `style`       |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[ruby]
symbol = "🔺 "
```

## Rust

By default the `rust` module shows the currently installed version of [Rust](https://www.rust-lang.org/). Module cho sẽ được hiện nếu bất kì điều kiện nào dưới đây thoả mãn:

- The current directory contains a `Cargo.toml` file
- The current directory contains a file with the `.rs` extension

### Các tuỳ chọn

| Tuỳ chọn            | Mặc định                             | Mô tả                                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Định dạng cho module.                                                     |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🦀 "`                               | A format string representing the symbol of Rust                           |
| `detect_extensions` | `["rs"]`                             | Những tiện ích mở rộng nào sẽ kích hoạt mô-đun này.                       |
| `detect_files`      | `["Cargo.toml"]`                     | Tên tệp nào sẽ kích hoạt mô-đun này.                                      |
| `detect_folders`    | `[]`                                 | Những thư mục nào sẽ kích hoạt mô-đun này.                                |
| `style`             | `"bold red"`                         | Kiểu cho module.                                                          |
| `disabled`          | `false`                              | Disables the `rust` module.                                               |

### Các biến

| Biến      | Ví dụ             | Mô tả                            |
| --------- | ----------------- | -------------------------------- |
| version   | `v1.43.0-nightly` | The version of `rustc`           |
| symbol    |                   | Giá trị ghi đè tuỳ chọn `symbol` |
| style\* |                   | Giá trị ghi đè của `style`       |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[rust]
format = "via [⚙️ $version](red bold)"
```

## Scala

The `scala` module shows the currently installed version of [Scala](https://www.scala-lang.org/). Mặc định module sẽ được hiển thị nếu có bất kì điều kiện nào dưới đây thoả mãn:

- The current directory contains a `build.sbt`, `.scalaenv` or `.sbtenv` file
- The current directory contains a file with the `.scala` or `.sbt` extension
- The current directory contains a directory named `.metals`

### Các tuỳ chọn

| Tuỳ chọn            | Mặc định                                 | Mô tả                                                                     |
| ------------------- | ---------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `"via [${symbol}(${version} )]($style)"` | Định dạng cho module.                                                     |
| `version_format`    | `"v${raw}"`                              | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `["sbt", "scala"]`                       | Những tiện ích mở rộng nào sẽ kích hoạt mô-đun này.                       |
| `detect_files`      | `[".scalaenv", ".sbtenv", "build.sbt"]`  | Tên tệp nào sẽ kích hoạt mô-đun này.                                      |
| `detect_folders`    | `[".metals"]`                            | Những thư mục nào nên kích hoạt các mô đun này.                           |
| `symbol`            | `"🆂 "`                                   | A format string representing the symbol of Scala.                         |
| `style`             | `"red dimmed"`                           | Kiểu cho module.                                                          |
| `disabled`          | `false`                                  | Disables the `scala` module.                                              |

### Các biến

| Biến      | Ví dụ    | Mô tả                            |
| --------- | -------- | -------------------------------- |
| version   | `2.13.5` | The version of `scala`           |
| symbol    |          | Giá trị ghi đè tuỳ chọn `symbol` |
| style\* |          | Giá trị ghi đè của `style`       |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[scala]
symbol = "🌟 "
```

## Shell

The `shell` module shows an indicator for currently used shell.

::: thử thuật

Mặc định, mô đun này được vô hiệu. Để kích hoạt nó, thiết lập `disabled` sang `false` trong tập tin cấu hình của bạn.

:::

### Các tuỳ chọn

| Tuỳ chọn               | Mặc định     | Mô tả                                                        |
| ---------------------- | ------------ | ------------------------------------------------------------ |
| `bash_indicator`       | `bsh`        | A format string used to represent bash.                      |
| `fish_indicator`       | `fsh`        | A format string used to represent fish.                      |
| `zsh_indicator`        | `zsh`        | A format string used to represent zsh.                       |
| `powershell_indicator` | `psh`        | A format string used to represent powershell.                |
| `ion_indicator`        | `ion`        | A format string used to represent ion.                       |
| `elvish_indicator`     | `esh`        | A format string used to represent elvish.                    |
| `tcsh_indicator`       | `tsh`        | A format string used to represent tcsh.                      |
| `unknown_indicator`    |              | The default value to be displayed when the shell is unknown. |
| `format`               | `$indicator` | Định dạng cho module.                                        |
| `disabled`             | `true`       | Disables the `shell` module.                                 |

### Các biến

| Biến      | Mặc định | Mô tả                                                      |
| --------- | -------- | ---------------------------------------------------------- |
| indicator |          | Mirrors the value of `indicator` for currently used shell. |

### Các vị dụ

```toml
# ~/.config/starship.toml

[shell]
fish_indicator = ""
powershell_indicator = "_"
unknown_indicator = "mystery shell"
disabled = false
```

## SHLVL

The `shlvl` module shows the current `SHLVL` ("shell level") environment variable, if it is set to a number and meets or exceeds the specified threshold.

### Các tuỳ chọn

| Tuỳ chọn    | Mặc định                     | Mô tả                                                         |
| ----------- | ---------------------------- | ------------------------------------------------------------- |
| `threshold` | `2`                          | Display threshold.                                            |
| `format`    | `"[$symbol$shlvl]($style) "` | Định dạng cho module.                                         |
| `symbol`    | `"↕️ "`                      | The symbol used to represent the `SHLVL`.                     |
| `repeat`    | `false`                      | Causes `symbol` to be repeated by the current `SHLVL` amount. |
| `style`     | `"bold yellow"`              | Kiểu cho module.                                              |
| `disabled`  | `true`                       | Disables the `shlvl` module.                                  |

### Các biến

| Biến      | Ví dụ | Mô tả                            |
| --------- | ----- | -------------------------------- |
| shlvl     | `3`   | The current value of `SHLVL`     |
| symbol    |       | Giá trị ghi đè tuỳ chọn `symbol` |
| style\* |       | Giá trị ghi đè của `style`       |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[shlvl]
disabled = false
format = "$shlvl level(s) down"
threshold = 3
```

## Singularity

The `singularity` module shows the current [Singularity](https://sylabs.io/singularity/) image, if inside a container and `$SINGULARITY_NAME` is set.

### Các tuỳ chọn

| Tuỳ chọn   | Mặc định                         | Mô tả                                            |
| ---------- | -------------------------------- | ------------------------------------------------ |
| `format`   | `'[$symbol\[$env\]]($style) '` | Định dạng cho module.                            |
| `symbol`   | `""`                             | A format string displayed before the image name. |
| `style`    | `"bold dimmed blue"`             | Kiểu cho module.                                 |
| `disabled` | `false`                          | Disables the `singularity` module.               |

### Các biến

| Biến      | Ví dụ        | Mô tả                            |
| --------- | ------------ | -------------------------------- |
| env       | `centos.img` | The current Singularity image    |
| symbol    |              | Giá trị ghi đè tuỳ chọn `symbol` |
| style\* |              | Giá trị ghi đè của `style`       |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[singularity]
format = '[📦 \[$env\]]($style) '
```

## Status

The `status` module displays the exit code of the previous command. The module will be shown only if the exit code is not `0`.

::: thử thuật

Mặc định, mô đun này được vô hiệu. Để kích hoạt nó, thiết lập `disabled` sang `false` trong tập tin cấu hình của bạn.

:::

::: warning This module is not supported on elvish shell. :::

### Các tuỳ chọn

| Tuỳ chọn                | Mặc định                      | Mô tả                                                |
| ----------------------- | ----------------------------- | ---------------------------------------------------- |
| `format`                | `"[$symbol$status]($style) "` | The format of the module                             |
| `symbol`                | `"✖"`                         | The symbol displayed on program error                |
| `not_executable_symbol` | `"🚫"`                         | The symbol displayed when file isn't executable      |
| `not_found_symbol`      | `"🔍"`                         | The symbol displayed when the command can't be found |
| `sigint_symbol`         | `"🧱"`                         | The symbol displayed on SIGINT (Ctrl + c)            |
| `signal_symbol`         | `"⚡"`                         | The symbol displayed on any signal                   |
| `style`                 | `"bold red"`                  | Kiểu cho module.                                     |
| `recognize_signal_code` | `true`                        | Enable signal mapping from exit code                 |
| `map_symbol`            | `false`                       | Enable symbols mapping from exit code                |
| `disabled`              | `true`                        | Disables the `status` module.                        |

### Các biến

| Biến           | Ví dụ   | Mô tả                                                                |
| -------------- | ------- | -------------------------------------------------------------------- |
| status         | `127`   | The exit code of the last command                                    |
| int            | `127`   | The exit code of the last command                                    |
| common_meaning | `ERROR` | Meaning of the code if not a signal                                  |
| signal_number  | `9`     | Signal number corresponding to the exit code, only if signalled      |
| signal_name    | `KILL`  | Name of the signal corresponding to the exit code, only if signalled |
| maybe_int      | `7`     | Contains the exit code number when no meaning has been found         |
| symbol         |         | Giá trị ghi đè tuỳ chọn `symbol`                                     |
| style\*      |         | Giá trị ghi đè của `style`                                           |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml

# ~/.config/starship.toml

[status]
style = "bg:blue"
symbol = "🔴"
format = '[\[$symbol $common_meaning$signal_name$maybe_int\]]($style) '
map_symbol = true
disabled = false

```

## Swift

By default the `swift` module shows the currently installed version of [Swift](https://swift.org/). Module cho sẽ được hiện nếu bất kì điều kiện nào dưới đây thoả mãn:

- The current directory contains a `Package.swift` file
- The current directory contains a file with the `.swift` extension

### Các tuỳ chọn

| Tuỳ chọn            | Mặc định                             | Mô tả                                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Định dạng cho module.                                                     |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🐦 "`                               | A format string representing the symbol of Swift                          |
| `detect_extensions` | `["swift"]`                          | Những tiện ích mở rộng nào sẽ kích hoạt mô-đun này.                       |
| `detect_files`      | `["Package.swift"]`                  | Tên tệp nào sẽ kích hoạt mô-đun này.                                      |
| `detect_folders`    | `[]`                                 | Những thư mục nào sẽ kích hoạt mô-đun này.                                |
| `style`             | `"bold 202"`                         | Kiểu cho module.                                                          |
| `disabled`          | `false`                              | Disables the `swift` module.                                              |

### Các biến

| Biến      | Ví dụ    | Mô tả                            |
| --------- | -------- | -------------------------------- |
| version   | `v5.2.4` | The version of `swift`           |
| symbol    |          | Giá trị ghi đè tuỳ chọn `symbol` |
| style\* |          | Giá trị ghi đè của `style`       |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[swift]
format = "via [🏎  $version](red bold)"
```

## Terraform

The `terraform` module shows the currently selected [Terraform workspace](https://www.terraform.io/docs/language/state/workspaces.html) and version.

::: thử thuật

By default the Terraform version is not shown, since this is slow for current versions of Terraform when a lot of plugins are in use. If you still want to enable it, [follow the example shown below](#with-terraform-version).

:::

Mặc định module sẽ được hiển thị nếu có bất kì điều kiện nào dưới đây thoả mãn:

- The current directory contains a `.terraform` folder
- Current directory contains a file with the `.tf` or `.hcl` extensions

### Các tuỳ chọn

| Tuỳ chọn            | Mặc định                             | Mô tả                                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol$workspace]($style) "` | The format string for the module.                                         |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"💠"`                                | A format string shown before the terraform workspace.                     |
| `detect_extensions` | `["tf", "hcl"]`                      | Những tiện ích mở rộng nào sẽ kích hoạt mô-đun này.                       |
| `detect_files`      | `[]`                                 | Tên tệp nào sẽ kích hoạt mô-đun này.                                      |
| `detect_folders`    | `[".terraform"]`                     | Những thư mục nào sẽ kích hoạt mô-đun này.                                |
| `style`             | `"bold 105"`                         | Kiểu cho module.                                                          |
| `disabled`          | `false`                              | Disables the `terraform` module.                                          |

### Các biến

| Biến      | Ví dụ      | Mô tả                            |
| --------- | ---------- | -------------------------------- |
| version   | `v0.12.24` | The version of `terraform`       |
| workspace | `default`  | The current Terraform workspace  |
| symbol    |            | Giá trị ghi đè tuỳ chọn `symbol` |
| style\* |            | Giá trị ghi đè của `style`       |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

#### With Terraform Version

```toml
# ~/.config/starship.toml

[terraform]
format = "[🏎💨 $version$workspace]($style) "
```

#### Without Terraform version

```toml
# ~/.config/starship.toml

[terraform]
format = "[🏎💨 $workspace]($style) "
```

## Thời gian

The `time` module shows the current **local** time. The `format` configuration value is used by the [`chrono`](https://crates.io/crates/chrono) crate to control how the time is displayed. Take a look [at the chrono strftime docs](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) to see what options are available.

::: thử thuật

Mặc định, mô đun này được vô hiệu. Để kích hoạt nó, thiết lập `disabled` sang `false` trong tập tin cấu hình của bạn.

:::

### Các tuỳ chọn

| Tuỳ chọn          | Mặc định                | Mô tả                                                                                                                              |
| ----------------- | ----------------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| `format`          | `"at [$time]($style) "` | The format string for the module.                                                                                                  |
| `use_12hr`        | `false`                 | Enables 12 hour formatting                                                                                                         |
| `time_format`     | see below               | The [chrono format string](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) used to format the time.                |
| `style`           | `"bold yellow"`         | The style for the module time                                                                                                      |
| `utc_time_offset` | `"local"`               | Sets the UTC offset to use. Range from -24 &lt; x &lt; 24. Allows floats to accommodate 30/45 minute timezone offsets. |
| `disabled`        | `true`                  | Disables the `time` module.                                                                                                        |
| `time_range`      | `"-"`                   | Sets the time range during which the module will be shown. Times must be specified in 24-hours format                              |

If `use_12hr` is `true`, then `time_format` defaults to `"%r"`. Otherwise, it defaults to `"%T"`. Manually setting `time_format` will override the `use_12hr` setting.

### Các biến

| Biến      | Ví dụ      | Mô tả                      |
| --------- | ---------- | -------------------------- |
| thời gian | `13:08:10` | The current time.          |
| style\* |            | Giá trị ghi đè của `style` |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[time]
disabled = false
format = '🕙[\[ $time \]]($style) '
time_format = "%T"
utc_time_offset = "-5"
time_range = "10:00:00-14:00:00"
```

## Username

The `username` module shows active user's username. Module cho sẽ được hiện nếu bất kì điều kiện nào dưới đây thoả mãn:

- The current user is root
- The current user isn't the same as the one that is logged in
- The user is currently connected as an SSH session
- The variable `show_always` is set to true

::: thử thuật

SSH connection is detected by checking environment variables `SSH_CONNECTION`, `SSH_CLIENT`, and `SSH_TTY`. If your SSH host does not set up these variables, one workaround is to set one of them with a dummy value.

:::

### Các tuỳ chọn

| Tuỳ chọn      | Mặc định                | Mô tả                                 |
| ------------- | ----------------------- | ------------------------------------- |
| `style_root`  | `"bold red"`            | The style used when the user is root. |
| `style_user`  | `"bold yellow"`         | The style used for non-root users.    |
| `format`      | `"[$user]($style) in "` | Định dạng cho module.                 |
| `show_always` | `false`                 | Always shows the `username` module.   |
| `disabled`    | `false`                 | Disables the `username` module.       |

### Các biến

| Biến    | Ví dụ        | Mô tả                                                                                       |
| ------- | ------------ | ------------------------------------------------------------------------------------------- |
| `style` | `"red bold"` | Mirrors the value of option `style_root` when root is logged in and `style_user` otherwise. |
| `user`  | `"matchai"`  | The currently logged-in user ID.                                                            |

### Ví dụ

```toml
# ~/.config/starship.toml

[username]
style_user = "white bold"
style_root = "black bold"
format = "user: [$user]($style) "
disabled = false
show_always = true
```

## Vagrant

The `vagrant` module shows the currently installed version of [Vagrant](https://www.vagrantup.com/). Mặc định module sẽ được hiển thị nếu có bất kì điều kiện nào dưới đây thoả mãn:

- The current directory contains a `Vagrantfile` file

### Các tuỳ chọn

| Tuỳ chọn            | Mặc định                             | Mô tả                                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Định dạng cho module.                                                     |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"⍱ "`                               | A format string representing the symbol of Vagrant.                       |
| `detect_extensions` | `[]`                                 | Những tiện ích mở rộng nào sẽ kích hoạt mô-đun này.                       |
| `detect_files`      | `["Vagrantfile"]`                    | Tên tệp nào sẽ kích hoạt mô-đun này.                                      |
| `detect_folders`    | `[]`                                 | Những thư mục nào sẽ kích hoạt mô-đun này.                                |
| `style`             | `"cyan bold"`                        | Kiểu cho module.                                                          |
| `disabled`          | `false`                              | Disables the `vagrant` module.                                            |

### Các biến

| Biến      | Ví dụ            | Mô tả                            |
| --------- | ---------------- | -------------------------------- |
| version   | `Vagrant 2.2.10` | The version of `Vagrant`         |
| symbol    |                  | Giá trị ghi đè tuỳ chọn `symbol` |
| style\* |                  | Giá trị ghi đè của `style`       |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[vagrant]
format = "via [⍱ $version](bold white) "
```

## VLang

The `vlang` module shows you your currently installed version of V. By default the module will be shown if any of the following conditions are met:
- The current directory contains a file with `.v` extension
- The current directory contains a `v.mod`, `vpkg.json` or `.vpkg-lock.json` file

### Các tuỳ chọn

| Tuỳ chọn            | Mặc định                                     | Mô tả                                               |
| ------------------- | -------------------------------------------- | --------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`         | Định dạng cho module.                               |
| `symbol`            | `"V "`                                       | A format string representing the symbol of V        |
| `detect_extensions` | `["v"]`                                      | Những tiện ích mở rộng nào sẽ kích hoạt mô-đun này. |
| `detect_files`      | `["v.mod", "vpkg.json", ".vpkg-lock.json" ]` | Tên tệp nào sẽ kích hoạt mô-đun này.                |
| `detect_folders`    | `[]`                                         | Những thư mục nào sẽ kích hoạt mô-đun này.          |
| `style`             | `"blue bold"`                                | Kiểu cho module.                                    |
| `disabled`          | `false`                                      | Disables the `vlang` module.                        |

### Các biến

| Biến      | Ví dụ  | Mô tả                            |
| --------- | ------ | -------------------------------- |
| version   | `v0.2` | The version of `v`               |
| symbol    |        | Giá trị ghi đè tuỳ chọn `symbol` |
| style\* |        | Giá trị ghi đè của `style`       |

### Ví dụ

```toml
# ~/.config/starship.toml
[v]
format = "via [V $version](blue bold) "
```

## VCSH

The `vcsh` module displays the current active [VCSH](https://github.com/RichiH/vcsh) repository. The module will be shown only if a repository is currently in use.

### Các tuỳ chọn

| Tuỳ chọn   | Mặc định                         | Mô tả                                                  |
| ---------- | -------------------------------- | ------------------------------------------------------ |
| `symbol`   |                                  | The symbol used before displaying the repository name. |
| `style`    | `"bold yellow"`                  | Kiểu cho module.                                       |
| `format`   | `"vcsh [$symbol$repo]($style) "` | Định dạng cho module.                                  |
| `disabled` | `false`                          | Disables the `vcsh` module.                            |

### Các biến

| Biến      | Ví dụ                                       | Mô tả                            |
| --------- | ------------------------------------------- | -------------------------------- |
| repo      | `dotfiles` if in a VCSH repo named dotfiles | The active repository name       |
| symbol    |                                             | Giá trị ghi đè tuỳ chọn `symbol` |
| style\* | `black bold dimmed`                         | Giá trị ghi đè của `style`       |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[vcsh]
format = "[🆅 $repo](bold blue) "
```

## Zig

By default the the `zig` module shows the currently installed version of [Zig](https://ziglang.org/). Module cho sẽ được hiện nếu bất kì điều kiện nào dưới đây thoả mãn:

- The current directory contains a `.zig` file

### Các tuỳ chọn

| Tuỳ chọn            | Mặc định                             | Mô tả                                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Định dạng cho module.                                                     |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"↯ "`                               | The symbol used before displaying the version of Zig.                     |
| `style`             | `"bold yellow"`                      | Kiểu cho module.                                                          |
| `disabled`          | `false`                              | Disables the `zig` module.                                                |
| `detect_extensions` | `["zig"]`                            | Những tiện ích mở rộng nào sẽ kích hoạt mô-đun này.                       |
| `detect_files`      | `[]`                                 | Tên tệp nào sẽ kích hoạt mô-đun này.                                      |
| `detect_folders`    | `[]`                                 | Những thư mục nào sẽ kích hoạt mô-đun này.                                |

### Các biến

| Biến      | Ví dụ    | Mô tả                            |
| --------- | -------- | -------------------------------- |
| version   | `v0.6.0` | The version of `zig`             |
| symbol    |          | Giá trị ghi đè tuỳ chọn `symbol` |
| style\* |          | Giá trị ghi đè của `style`       |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

### Ví dụ

```toml
# ~/.config/starship.toml

[zig]
symbol = "⚡️ "
```

## Custom commands

The `custom` modules show the output of some arbitrary commands.

These modules will be shown if any of the following conditions are met:

- The current directory contains a file whose name is in `files`
- The current directory contains a directory whose name is in `directories`
- The current directory contains a file whose extension is in `extensions`
- The `when` command returns 0

::: thử thuật

Multiple custom modules can be defined by using a `.`.

:::

::: thử thuật

The order in which custom modules are shown can be individually set by including `${custom.foo}` in the top level `format` (as it includes a dot, you need to use `${...}`). By default, the `custom` module will simply show all custom modules in the order they were defined.

:::

::: thử thuật

[Issue #1252](https://github.com/starship/starship/discussions/1252) contains examples of custom modules. If you have an interesting example not covered there, feel free to share it there!

:::

### Các tuỳ chọn

| Tuỳ chọn      | Mặc định                        | Mô tả                                                                                                                      |
| ------------- | ------------------------------- | -------------------------------------------------------------------------------------------------------------------------- |
| `command`     |                                 | The command whose output should be printed. The command will be passed on stdin to the shell.                              |
| `when`        |                                 | A shell command used as a condition to show the module. The module will be shown if the command returns a `0` status code. |
| `shell`       |                                 | [See below](#custom-command-shell)                                                                                         |
| `description` | `"<custom module>"`       | The description of the module that is shown when running `starship explain`.                                               |
| `files`       | `[]`                            | The files that will be searched in the working directory for a match.                                                      |
| `directories` | `[]`                            | The directories that will be searched in the working directory for a match.                                                |
| `extensions`  | `[]`                            | The extensions that will be searched in the working directory for a match.                                                 |
| `symbol`      | `""`                            | The symbol used before displaying the command output.                                                                      |
| `style`       | `"bold green"`                  | Kiểu cho module.                                                                                                           |
| `format`      | `"[$symbol($output )]($style)"` | Định dạng cho module.                                                                                                      |
| `disabled`    | `false`                         | Disables this `custom` module.                                                                                             |

### Các biến

| Biến      | Mô tả                                  |
| --------- | -------------------------------------- |
| output    | The output of shell command in `shell` |
| symbol    | Giá trị ghi đè tuỳ chọn `symbol`       |
| style\* | Giá trị ghi đè của `style`             |

\*: Biến này có thể chỉ được sử dụng như một phần của style string

#### Custom command shell

`shell` accepts a non-empty list of strings, where:

- The first string is the path to the shell to use to execute the command.
- Other following arguments are passed to the shell.

If unset, it will fallback to STARSHIP_SHELL and then to "sh" on Linux, and "cmd /C" on Windows.

The `command` will be passed in on stdin.

If `shell` is not given or only contains one element and Starship detects PowerShell will be used, the following arguments will automatically be added: `-NoProfile -Command -`. This behavior can be avoided by explicitly passing arguments to the shell, e.g.

```toml
shell = ["pwsh", "-Command", "-"]
```

::: warning Make sure your custom shell configuration exits gracefully

If you set a custom command, make sure that the default Shell used by starship will properly execute the command with a graceful exit (via the `shell` option).

For example, PowerShell requires the `-Command` parameter to execute a one liner. Omitting this parameter might throw starship into a recursive loop where the shell might try to load a full profile environment with starship itself again and hence re-execute the custom command, getting into a never ending loop.

Parameters similar to `-NoProfile` in PowerShell are recommended for other shells as well to avoid extra loading time of a custom profile on every starship invocation.

Automatic detection of shells and proper parameters addition are currently implemented, but it's possible that not all shells are covered. [Please open an issue](https://github.com/starship/starship/issues/new/choose) with shell details and starship configuration if you hit such scenario.

:::

### Ví dụ

```toml
# ~/.config/starship.toml

[custom.foo]
command = "echo foo"  # shows output of command
files = ["foo"]       # can specify filters
when = """ test "$HOME" == "$PWD" """
format = " transcending [$output]($style)"

[custom.time]
command = "time /T"
files = ["*.pst"]
shell = ["pwsh.exe", "-NoProfile", "-Command", "-"]
```
