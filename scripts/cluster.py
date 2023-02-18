import json

import numpy as np
from nltk.corpus import wordnet as wn
from scipy.sparse import csr_matrix
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


def clean_topics(topics):
    """
    Filtering out any empty topics and return list of non-empty topics for each repository,

    Then generate a corpus of text for the TfidfVectorizer to process. Clean up the topics data.
    Each element in the corpus is a sentence containing the topic(s) for a single repository.
    """
    corpus = []
    for topic in topics:
        if len(topic) > 0:
            corpus.append(" ".join(topic))
    return corpus


# Create feature matrix.
def process_feature_matrix(corpus):
    vectorizer = TfidfVectorizer()
    X = vectorizer.fit_transform(corpus)
    return X


# Learn vocabulary and idf, return document-term matrix.
def main():
    # Extract the topics from the repository data.
    topics = extract_topics(PATH_PARSED_JSON)

    # Clean up the topics data.
    group_topics = clean_topics(topics)
    print(json.dumps(group_topics, indent=4))


#   # Create a feature matrix.
#   X = process_feature_matrix(group_topics)

#   # Convert the output you provided into a sparse matrix
#   sparse_matrix = np.array(X)  # sparse_matrix or topic_weights
#   print(X, sparse_matrix)

#   # Traceback (most recent call last):
#   #   File "/home/lloyd/p/dashboard/scripts/cluster.py", line 90, in <module>
#   #     data = main()
#   #            ^^^^^^
#   #   File "/home/lloyd/p/dashboard/scripts/cluster.py", line 79, in main
#   #     for i in range(len(sparse_matrix)):
#   #                    ^^^^^^^^^^^^^^^^^^
#   # TypeError: len() of unsized object
#   # Generate a list of tuples where each tuple contains the document index followed by the indices of the top two topics for that document.
#   top_topics = []
#   for i in range(len(sparse_matrix)):
#       sorted_topics = sorted(
#           enumerate(sparse_matrix[i]), key=lambda x: x[1], reverse=True
#       )
#       top_topics.append((i, sorted_topics[0][0], sorted_topics[1][0]))
#   pass

#   return X


if __name__ == "__main__":
    data = main()
    print(data)
