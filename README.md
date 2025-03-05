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

- Rust 1.8.0

- Redis database

- Environment configuration files (.env)

## Deployment Instructions

1. Clone the repository:

    `git clone https://github.com/northern05/ai-agent.git
    cd ai-agent`

2. Configure your environment:

   - Add necessary credentials and configurations in the `.envs/.dev` file.


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


---

## **Shilling is not allowed when an agent engages with a user without using the designated button.**

- `identifyPool`
    
    **Description**: Fetch address of a given pool address from a Raydium URL or an exact pool address or token address to identify if this pair pool exists on Raydium.
    
    **Properties** (params used): 
    
    1. `pool_or_token_address`
    
    **Description**: Fetch token address or pool address or pool address from Raydium url.
    
- `retrieveCurrentPortfolio`
    
    **Description**: Retrieve information about the current agent's portfolio, including the tokens it holds.
    
- `retrieveBuyExplanation`
    
    **Description**: Retrieve explanation why you bought specific meme token.
    
    **Properties**: 
    
    1. `pool_address`
        
        **Description:** Retrieve the pool address of token pair, but first call retrieveCurrentPortfolio to identify the pool address by name.
        
- `retrievePnlInformation`
    
    **Description**: Retrieve profit and loss (PnL) statistics based on the user's requested action.
    
    **Properties**: 
    
    1. `action`
        
        **Description:** Specify the action to retrieve statistics related to profit and loss (PnL) or agent performance.
        
        Enum: 
        
        `total_pnl` - information about total pay and losses  
        `total_profit_shared` - information about total shared profit between users  
        `maximum_pnl` - information about the most profitable trade  
        `minimum_pnl` - information about the most losses trade  
        `average_pnl` - information about the average PNL of agent trading  
        `count_of_trades` - information about how many trades agent has closed  

---

## **Shilling is not allowed, but this function call is used to identify the pool by filtering the prompt to fix a bug that causes duplication of logic and incorrect data fetching.**

- `identifyPool`
    
    **Description**: Fetch address of a given pool address from a Raydium URL or an exact pool address or token address to identify if this pair pool exists on Raydium.
    
    **Properties** (params used): 
    
    1. `pool_or_token_address`
    
    **Description**: Fetch token address or pool address or pool address from Raydium url.
    
- `analyzeCallIdentifyPool`
    
    **Description**: Analyze user message if there address of token to call identifyPool function.
    
    **Properties**: 
    
    1. `is_function_call`
        
        **Description**: Boolean value to check if user message contains address or pool address or link.
        
    2. `user_message`
        
        **Description**: User message to analyze.

---

## **Shilling is allowed**

- `approveShilling`
    
    **Description**: Approve buying meme token from Raydium explanation.
    
    **Properties**: 
    
    1. `explanation`
        
        **Description**: Explanation for why you decide to buy token meme that user provided from Raydium.
        
    2. `poolAddress`
        
        **Description**: Extract the poolAddress of pair provided in analytic data (which you previously called in the function fetch_pool_data from your message history)
        
- `rejectShilling`
    
    **Description**: Reject buying meme token from Raydium and provide explanation.
    
    **Properties**: 
    
    1. `explanation`
        
        **Description**: Explanation for why you decide to buy token meme that user provided from Raydium.
        
    2. `poolAddress`
        
        **Description**: Extract the poolAddress of pair provided in analytic data (which you previously called in the function fetch_pool_data from your message history)
        
- `fetch_pool_data`
    
    **Description**: Fetch analytics data for a given token address.
    
    **Properties**: 
    
    1. `token_address`
        
        **Description**: The address of the pool to fetch data for.

---

## **Twitter function call**

- `generatePostInTwitter`
    
    **Description**: Generate text and post in twitter account.
    
    **Properties**: 
    
    1. `data`
        
        **Description**: Data provided to generate post in twitter account.
---

# Technologies Used

- **Blockchain Integration**: Interacts with Ethereum smart contracts.

- **Redis**: Stores chat histories for efficient retrieval and persistence.

- **LLM (Language Model)**: Processes user messages and makes decisions.

- **Rust**: Core programming language.

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

For any questions or issues, please contact the repository owner via the TG: @yezhovanton
