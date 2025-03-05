# AI Agent with Bonding Curve Business Logic

## Overview

This repository implements the business logic for an AI-powered agent that interacts with a Bonding Curve Contract to manage token purchases. The system uses an AI agent to validate user prompts and make decisions on token swaps, based on user interaction and the contract's state.

## Key Features:

**Integration with Bonding Curve Contract**:
- Connects to the Bonding Curve contract.

**Prompt Message Validation**: 
- Ensures message integrity by hashing the user's message and comparing it with the pre-hashed message from the transaction.

**ETH-Based Chat Sessions**:
- Creates a unique chat session for each Ethereum address.
- Users must convince the AI agent to approve token purchases based on their ETH balance in the contract.

**AI-Driven Swap Decisions**:
- Sends the user's last message to a language model (LLM) for evaluation.
- Receives the LLM's decision (approve or reject) through function calls.

**Transaction Processing**:
- If approved by the LLM, the system executes the transaction and sends the result to the frontend.

**Chat History Management**: 
    - All chat sessions are stored in a Redis database for tracking and future references.

# Project Structure

## Business Logic

- **Contract Integration**: Connects with the Bonding Curve smart contract to validate ETH balances and process transactions.

- **Message Hashing**: Ensures message authenticity using cryptographic hashing.

- **AI Decision Making**:

  - Submits user messages to an LLM.

  - Processes and interprets LLM's responses to determine the course of action.

- **Transaction Execution**: Automates token swaps upon approval and communicates the outcome to the frontend.

- **Redis Chat History**: Stores every user's chat session for a seamless and persistent experience.

# Setup and Deployment

## Prerequisites

- Python 3.8+

- Redis database

- Environment configuration files (.env)

## Deployment Instructions

1. Clone the repository:

    `git clone https://github.com/northern05/ai-agent.git
    cd ai-agent`

2. Configure your environment:

   - Add necessary credentials and configurations in the `.envs/.dev` file.

3. Deploy the project:

`sh deploy.sh dev`

# How It Works

1. **Initialization**:

    - The system connects to the Bonding Curve contract upon startup.

    - A Redis database is initialized to store chat histories.

2. **User Interaction**:

    - Users initiate a chat session by sending a message.

    - The system hashes the message and validates it against the pre-hashed message from the contract.

3. **AI Validation**:

    - The user’s message is sent to the LLM for evaluation.

    - The AI agent decides whether to approve or reject the token purchase based on the user’s message and ETH balance.

4. **Transaction Execution**:

    - If the LLM approves, the system executes the token swap transaction and sends the details to the frontend.

    - If rejected, the user is informed with the AI’s reasoning.

5. Data Storage:

    - All chat histories are saved in the Redis database for reference.

# Technologies Used

- **Blockchain Integration**: Interacts with Ethereum smart contracts.

- **Redis**: Stores chat histories for efficient retrieval and persistence.

- **LLM (Language Model)**: Processes user messages and makes decisions.

- **Python**: Core programming language.

# Contributing

1. Fork the repository.

2. Create a new branch:

    `git checkout -b feature-name`

3. Commit your changes:

    `git commit -m 'Add some feature'`

4. Push to the branch:

    `git push origin feature-name`

5. Submit a pull request.

# License

This project is licensed under the MIT License. See the LICENSE file for details.

# Contact

For any questions or issues, please contact the repository owner via the [GitHub repo](https://github.com/northern05/ai-agent).