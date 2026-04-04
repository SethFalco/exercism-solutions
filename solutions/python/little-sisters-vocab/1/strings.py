"""Functions for creating, transforming, and adding prefixes to strings."""


def add_prefix_un(word: str) -> str:
    """
    Take the given word and add the 'un' prefix.

    :param word: containing the root word.
    :return: of root word prepended with 'un'.
    """
    return "un" + word


def make_word_groups(vocab_words: list[str]) -> str:
    """
    Transform a list containing a prefix and words into a string with the prefix followed by the words with prefix prepended.

    :param vocab_words: of vocabulary words with prefix in first index.
    :return: of prefix followed by vocabulary words with prefix applied.

    This function takes a `vocab_words` list and returns a string
    with the prefix and the words with prefix applied, separated
     by ' :: '.

    For example: list('en', 'close', 'joy', 'lighten'),
    produces the following string: 'en :: enclose :: enjoy :: enlighten'.
    """
    prefix = vocab_words[0]
    new_arr = [ prefix + word for word in vocab_words[1:] ]
    new_arr.insert(0, prefix)
    return " :: ".join(new_arr)


def remove_suffix_ness(word: str) -> str:
    """
    Remove the suffix from the word while keeping spelling in mind.

    :param word: of word to remove suffix from.
    :return: of word with suffix removed & spelling adjusted.

    For example: "heaviness" becomes "heavy", but "sadness" becomes "sad".
    """
    word = word[0:-4]

    if word[-1] == "i":
        word = word[0:-1] + "y"

    return word


def adjective_to_verb(sentence: str, index: int) -> str:
    """
    Change the adjective within the sentence to a verb.

    :param sentence: that uses the word in sentence.
    :param index: index of the word to remove and transform.
    :return: word that changes the extracted adjective to a verb.

    For example, ("It got dark as the sun set", 2) becomes "darken".
    """
    sentence_no_punc = sentence[0:-1]
    words = sentence_no_punc.split(" ")
    return words[index] + "en"
