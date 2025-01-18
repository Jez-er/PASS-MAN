# PASS MAN | Password Manager 🔐

**PASS MAN** is a CLI utility for password generation, local storage, and easy management through the command line. With this tool, you can not only generate secure passwords but also store them in one place to easily manage and access your credentials directly from the console.

---

## 🚀 Key Features

- **Password Generation**: Easily create passwords with varying degrees of complexity — from simple to highly complex ones, using characters, numbers, and special symbols.
- **Password Storage**: All passwords are stored in an encrypted local file, ensuring their security.
- **Password Management**: Conveniently add, remove, and view stored passwords via the command-line interface.
- **Flexibility**: Ability to generate passwords with various parameters (length, presence of digits, symbols, etc.).

<!-- ---

## 📦 Installation

To use **PASS MAN**, you need Python 3.x. To install and run the utility, follow these steps:

1. Clone the repository:

    ```bash
    git clone https://github.com/yourusername/pass-man.git
    ```

2. Navigate to the project directory:

    ```bash
    cd pass-man
    ```

3. Install dependencies:

    ```bash
    pip install -r requirements.txt
    ```

4. Run the utility:

    ```bash
    python passman.py
    ```

--- -->

## 🛠️ Usage

### Password Generation

**PASS MAN** allows you to generate two types of passwords:

1. **Simple Password** (Only letters and numbers):

   ```bash
   gen_simple | gs
   ```

2. **Complex Password** (Letters, numbers, and special symbols):

   ```bash
   gen_pass | gp
   ```

<!-- ### Password Storage

After generating a password, you can save it to the secure storage with the following command:

```bash
passman save "service_name" "generated_password"
```

To view all saved passwords:

```bash
passman list
```

To delete a password:

```bash
passman delete "service_name"
```

--- -->

## 💡 Tasks

Here are the tasks and their statuses.

| Task                                          | Status         |
| --------------------------------------------- | -------------- |
| Generate simple passwords.                    | ✅ Done        |
| Generating passwords with special characters. | ✅ Done        |
| Connect a local database.                     | ✅ Done        |
| Authorization.                                | ✅ Done        |
| Saving password.                              | ✅ Done        |
| Password encryption.                          | ✅ Done        |
| Adding parameters to commands                 | ✅ Done        |
| Getting a password.                           | ✅ Done        |
| Getting list of the all passwords.            | ❌ In progress |
| Edit password.                                | ❌ Not Started |
| Delete password.                              | ❌ Not Started |
| Create help command.                          | ❌ Not Started |
| Profile password hashing.                     | ❌ Not Started |

---

## 🔐 Security

- All passwords are stored in an encrypted form using the AES algorithm.

- Never store your master password in plain text! Use a secure method to store it if necessary.

---

<!-- ## 📄 License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

--- -->

## ✨ Contributing

We welcome your improvements! If you have ideas or fixes, please open a pull request or report issues in the Issues section.

---

## 📬 Contact

If you have any questions or suggestions, feel free to reach out:

- Email: andrey.chernenko208@gmail.com
- GitHub: [@Jez-er](https://github.com/Jez-er)
- Telegram: [@Jeze_1](https://t.me/jeze_1)
