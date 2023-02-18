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

HELP_FIELDS = ",".join(
    [
        "created_at",
        "description",
        "disk_usage",
        "id",
        "name",
        "pushed_at",
        "repository_topics",
        "ssh_url",
        "stargazer_count",
        "updated_at",
        "url",
    ]
).strip()


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


def printf(any_data):
    """Print data as pretty-printed JSON."""
    print(json.dumps(any_data, indent=4, cls=ListEncoder))


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


def extract_repo_data_cli(
    repos: List[GitHubRepository], fields: List[str]
) -> List[dict]:
    """
    Extracts data from a list of GitHubRepository instances based on a list of field names.

    Args:
        repos (List[GitHubRepository]): A list of GitHubRepository instances to extract data from.
        fields (List[str]): A list of field names to extract from each repository.

    Returns:
        List[dict]: A list of dictionaries containing extracted data for each repository.
    """
    data = []
    for repo in repos:
        new_list = {}
        for field in fields:
            if hasattr(repo, field):
                new_list[field] = getattr(repo, field)
            else:
                print(f"Field '{field}' not found in GitHubRepository class.")
        data.append(new_list)
    if len(data) != len(repos):
        print("Error while extracting")
    else:
        pass
    return data


################################################################################


# The `main` function uses the argparse module to parse command-line arguments,
# and then it uses the append_repositories function to read repository data
# from the README.json file. If the user specifies a list of fields to extract
# using the --json argument, the function calls the extract_repo_data_cli
# function with the specified fields. Otherwise, the function calls the
# extract_repo_data function to extract all fields.
#
# The function returns a list of dictionaries containing the extracted data.
# If the --json argument was used, the dictionaries will contain only the
# specified fields. If no fields were specified, the dictionaries will
# contain all fields. Finally, the function uses the json module to
# pretty-print the output as a JSON-formatted string.
def main() -> List[dict]:
    """
    The main function of the script. This function can also parses command-line arguments
    and uses them to extract data from GitHub repositories.

    Returns:
        List[dict]: A list of dictionaries containing custom repository data.
    """
    import argparse

    parser = argparse.ArgumentParser(
        description="Extract data from GitHub repositories",
        # formatter_class=argparse.RawTextHelpFormatter,
    )
    parser.add_argument(
        "--json",
        metavar="N",
        type=str,
        nargs="+",
        help=f"Specify one or more comma-separated fields for `--json`: [\
               {HELP_FIELDS}\
               ]",
    )

    args = parser.parse_args()
    repositories = append_repositories("README.json")

    if args.json:
        fields = [field.strip() for field in args.json[0].split(",")]
        data = extract_repo_data_cli(repositories, fields=fields)
    else:  # No fields specified, return default.
        data = extract_repo_data(repositories)
    return data


################################################################################

if __name__ == "__main__":
    data = main()
    print(json.dumps(data, indent=4, cls=ListEncoder))

################################################################################

# Use dir() to get a list of available fields in the GitHubRepository class
# available_fields = [f for f in dir(
#     GitHubRepository) if not f.startswith("__")]
# print(available_fields)
# fields_help_str = "\n".join(available_fields)
# available_fields = dir(GitHubRepository)
# available_fields.remove("__doc__")  # Remove __doc__ field
# for item in GitHubRepository.ke
#     print(item)
# for f in available_fields:
#     print(f)
