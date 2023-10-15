def problem_definition():
    return '''XOR decryption

    Each character on a computer is assigned a unique code and the preferred standard is ASCII
    (American Standard Code for Information Interchange). For example, uppercase A = 65, asterisk (*) = 42,
    and lowercase k = 107.

    A modern encryption method is to take a text file, convert the bytes to ASCII, then XOR each byte with
    a given value, taken from a secret key. The advantage with the XOR function is that using the same
    encryption key on the cipher text, restores the plain text; for example, 65 XOR 42 = 107, then 107 XOR 42 = 65.

    For unbreakable encryption, the key is the same length as the plain text message, and the key is made up
    of random bytes. The user would keep the encrypted message and the encryption key in different locations,
    and without both "halves", it is impossible to decrypt the message.

    Unfortunately, this method is impractical for most users, so the modified method is to use a password as
    a key. If the password is shorter than the message, which is likely, the key is repeated cyclically
    throughout the message. The balance for this method is using a sufficiently long password key for security,
    but short enough to be memorable.

    Your task has been made easy, as the encryption key consists of three lower case characters.
    Using cipher.txt (right click and 'Save Link/Target As...'), a file containing the encrypted ASCII codes,
    and the knowledge that the plain text must contain common English words, decrypt the message and find the
    sum of the ASCII values in the original text.'''


with open('resources/p059_cipher.txt', encoding='utf-8') as file:
    encrypted_data = ''.join(chr(int(n)) for n in file.readline().split(','))


def encrypt(string, password):
    # For each character, XOR with the password (repeat the password when it exceeds its length)
    pw_len = len(password)
    return ''.join(chr(ord(string[i]) ^ ord(password[i % pw_len])) for i in range(len(string)))


def work():
    print('Searching...')
    start = ord('a')
    end = ord('z') + 1
    for x in range(start, end):
        print('Checking passwords starting by {}'.format(chr(x)))
        for y in range(start, end):
            for z in range(start, end):
                pw = chr(x) + chr(y) + chr(z)
                decrypted = encrypt(encrypted_data, pw)
                if 'because' in decrypted:
                    print('Looks OK with password {}:'.format(pw))
                    print(decrypted)
                    print('The sum of the ASCII value of characters is {}'
                          .format(sum(ord(c) for c in decrypted)))

                    if input('Stop now (y/n)?: ') == 'y':
                        return

work()
print('Done.')
