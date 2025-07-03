from setuptools import setup

setup(
    name="ai_assistant",
    version="0.1.0",
    packages=["ai_assistant"],
    install_requires=[
        "openai==1.3.0",
        "python-dotenv==1.0.0",
        "requests==2.32.4"
    ],
    entry_points={
        'console_scripts': [
            'ai_assistant=ai_assistant.__main__:main'
        ]
    }
)
