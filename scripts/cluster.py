import json

import numpy as np  # Pyright: "np" is not accessed
from nltk.corpus import wordnet as wn
from sklearn.feature_extraction.text import TfidfVectorizer

PATH_PARSED_JSON = "parsed.json"


# Read repository data from a JSON file
def read_repositories(path):
    repositories = []
    with open(path) as f:
        for item in json.load(f):
            repositories.append(item)
    return repositories


# Extract topic data from repositories.
def extract_topics(path):
    repositories = read_repositories(path)
    topics = [
        repo["repository_topics"] for repo in repositories
    ]  # topics = [repo.repository_topics for repo in repositories]
    return topics


# Clean up the topics data.
def clean_topics(topics):
    some_topics = []
    for topic in topics:
        if len(topic) > 0:
            some_topics.append(topic)
        # else:
        # some_topics.append(None)
    return some_topics


corpus = [
    "This is the first document.",
    "This document is the second document.",
    "And this is the third one.",
    "Is this the first document?",
]


# Create feature matrix.
def process_feature_matrix(topics):
    vectorizer = TfidfVectorizer()
    X = vectorizer.fit_transform(topics)
    return X


# Learn vocabulary and idf, return document-term matrix.
def main():
    # Extract the topics from the repository data.
    topics = extract_topics(PATH_PARSED_JSON)

    # Clean up the topics data.
    some_topics = clean_topics(topics)

    group_topics = []
    for items in some_topics:
        for item in items:
            group_topics.append(item)
    pass
    print(json.dumps(group_topics, indent=4))

    # Create a feature matrix.
    X = process_feature_matrix(group_topics)

    return X


if __name__ == "__main__":
    data = main()
    print(data)

# from typing import List
# from typing import Optional
