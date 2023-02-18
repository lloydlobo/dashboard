"""
parse.py or (github_repository.py): A Python program to extract custom data from GitHub repository information.

This program reads a JSON file that contains information about GitHub repositories and extracts specific
data for each repository. It then returns the extracted data as a list of dictionaries.

This program defines two classes: Repository and GitHubRepository. The former is used as a helper class
to hold repository data, while the latter is used to represent a GitHub repository.

The program first reads the JSON file using the append_repositories function. This function takes a file
path as input, opens the file, reads the contents, and returns a list of GitHubRepository objects.

The program then uses the extract_repo_data function to extract specific data from the list of GitHubRepository
objects. This function takes a list of GitHubRepository objects as input and returns a list of dictionaries
that contain the specific data for each repository.

The ListEncoder class is a custom JSON encoder that is used to encode the Repository and GitHubRepository objects
as dictionaries so they can be serialized to JSON.

The program also defines a printf function that takes any data and prints it to the console as pretty-printed JSON.

Finally, the program defines a main function that calls the append_repositories and extract_repo_data functions,
and returns the resulting data as a list of dictionaries. If the program is run directly, it will call the main
function and print the resulting data as pretty-printed JSON to the console.

This program is compatible with Python 3.6 or later.
"""
import json
from typing import List
from typing import Optional


################################################################################


class Repository:
    """
    Represents a generic repository.

    Args:
        name (Optional[str]): The name of the repository.
        url (Optional[str]): The URL of the repository.
        description (Optional[str]): A description of the repository.
        repository_topics (Optional[str]): A list of topics associated with the repository.
    """

    def __init__(
        self,
        name: Optional[str] = None,
        url: Optional[str] = None,
        description: Optional[str] = None,
        repository_topics: Optional[str] = None,
    ):
        self.name = name
        self.url = url
        self.description = description
        self.repository_topics = repository_topics


class GitHubRepository:
    """
    Represents a GitHub repository.

    Args:
        created_at (str): The date and time the repository was created.
        description (str): A description of the repository.
        disk_usage (int): The disk usage of the repository.
        id (str): The ID of the repository.
        name (str): The name of the repository.
        pushed_at (str): The date and time of the most recent push to the repository.
        ssh_url (str): The SSH URL of the repository.
        stargazer_count (int): The number of users who have starred the repository.
        updated_at (str): The date and time the repository was last updated.
        url (str): The URL of the repository.
        repository_topics (Optional[List[str]]): A list of topics associated with the repository.
    """

    def __init__(
        self,
        created_at: str,
        description: str,
        disk_usage: int,
        id: str,
        name: str,
        pushed_at: str,
        ssh_url: str,
        stargazer_count: int,
        updated_at: str,
        url: str,
        repository_topics: Optional[List[str]] = None,
    ):
        self.created_at = created_at
        self.description = description
        self.disk_usage = disk_usage
        self.id = id
        self.name = name
        self.pushed_at = pushed_at
        self.repository_topics = repository_topics
        self.ssh_url = ssh_url
        self.stargazer_count = stargazer_count
        self.updated_at = updated_at
        self.url = url

    def __repr__(self):
        return (
            f"GitHubRepository("
            f"created_at={self.created_at!r}, description={self.description!r}, "
            f"disk_usage={self.disk_usage!r}, id={self.id!r}, name={self.name!r}, "
            f"pushed_at={self.pushed_at!r}, repository_topics={self.repository_topics!r}, "
            f"ssh_url={self.ssh_url!r}, stargazer_count={self.stargazer_count!r}, "
            f"updated_at={self.updated_at!r}, url={self.url!r})"
        )


################################################################################


class ListEncoder(json.JSONEncoder):
    """
    Encodes a list of objects as JSON.

    Args:
        json.JSONEncoder: The default JSON encoder.

    Returns:
        JSON: The JSON
    """

    def default(self, obj):
        if isinstance(obj, (Repository, GitHubRepository)):
            return obj.__dict__
        return super().default(obj)


################################################################################


def parse_map_topics(item: dict) -> Optional[List[str]]:
    """
    Extracts topics from a dictionary.

    Args:
        item (dict): A dictionary containing topics.

    Returns:
        Optional[List[str]]: A list of topics, or None if there are no topics.
    """
    topics = item.get("repositoryTopics")
    if not topics:
        return []
    return [t["name"] for t in topics]


################################################################################


def append_repositories(path):
    """
    Reads repository data from a JSON file and returns a list of GitHubRepository objects.

    Args:
        path (str): The path to the JSON file containing repository data.

    Returns:
        List[GitHubRepository]: A list of GitHubRepository objects.

    """
    repositories = []
    with open(path) as f:
        for item in json.load(f):
            repo = GitHubRepository(
                created_at=item["createdAt"],
                description=item["description"],
                disk_usage=item["diskUsage"],
                id=item["id"],
                name=item["name"],
                pushed_at=item["pushedAt"],
                repository_topics=parse_map_topics(item),
                ssh_url=item["sshUrl"],
                stargazer_count=item["stargazerCount"],
                updated_at=item["updatedAt"],
                url=item["url"],
            )
            repositories.append(repo)
        pass
    return repositories


################################################################################


def printf(any_data):
    print(json.dumps(any_data, indent=4, cls=ListEncoder))


def extract_repo_data(repos: List[GitHubRepository]) -> List[dict]:
    """
    Extracts custom repository data from a list of GitHubRepository objects.

    Args:
        repos (List[GitHubRepository]): A list of GitHubRepository objects.

    Returns:
        List[dict]: A list of dictionaries containing custom repository data.

    """
    data = []
    for repo in repos:
        new_list = {
            "name": repo.name,
            "url": repo.url,
            "description": repo.description,
            "repository_topics": repo.repository_topics,
        }
        data.append(new_list)
    if len(data) != len(repos):
        print("Error while extracting")
    else:  # print("Extracted custom data")
        pass
    return data


################################################################################


def main() -> List[dict]:
    """
    The main function of the script.

    Returns:
        List[dict]: A list of dictionaries containing custom repository data.

    """
    repositories = append_repositories("README.json")
    custom_data = extract_repo_data(repositories)
    return custom_data


if __name__ == "__main__":
    data = main()
    print(json.dumps(data, indent=4, cls=ListEncoder))
