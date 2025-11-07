/**
 * Quick Start Example for LLM Memory Graph Client
 *
 * This example demonstrates basic usage of the client library
 * including session management, prompts, responses, and queries.
 */

import { MemoryGraphClient, NodeType } from '../src';

async function main() {
  // Create a client instance
  console.log('Creating client...');
  const client = new MemoryGraphClient({
    address: 'localhost:50051',
    useTls: false,
    timeout: 30000,
  });

  try {
    // Check service health
    console.log('\n--- Checking Service Health ---');
    const health = await client.health();
    console.log('Service Status:', health.status);
    console.log('Version:', health.version);
    console.log('Uptime:', health.uptimeSeconds, 'seconds');

    // Create a new session
    console.log('\n--- Creating Session ---');
    const session = await client.createSession({
      metadata: {
        user: 'quickstart-example',
        application: 'demo',
        environment: 'development',
      },
    });
    console.log('Session created:', session.id);
    console.log('Created at:', session.createdAt);

    // Add a prompt to the session
    console.log('\n--- Adding Prompt ---');
    const prompt = await client.addPrompt({
      sessionId: session.id,
      content: 'What are the main features of TypeScript?',
      metadata: {
        model: 'gpt-4',
        temperature: 0.7,
        maxTokens: 500,
        toolsAvailable: ['search', 'code_interpreter'],
        custom: {
          priority: 'medium',
          category: 'programming',
        },
      },
    });
    console.log('Prompt created:', prompt.id);
    console.log('Content:', prompt.content);

    // Add a response to the prompt
    console.log('\n--- Adding Response ---');
    const response = await client.addResponse({
      promptId: prompt.id,
      content:
        'TypeScript is a typed superset of JavaScript with several key features:\n' +
        '1. Static typing for better code quality and IDE support\n' +
        '2. Advanced type inference and type checking\n' +
        '3. Object-oriented programming features like classes and interfaces\n' +
        '4. Modern JavaScript features (ES6+) with backward compatibility\n' +
        '5. Excellent tooling and editor support',
      tokenUsage: {
        promptTokens: 12,
        completionTokens: 85,
        totalTokens: 97,
      },
      metadata: {
        model: 'gpt-4',
        finishReason: 'stop',
        latencyMs: 2340,
        custom: {
          confidence: '0.95',
        },
      },
    });
    console.log('Response created:', response.id);
    console.log('Token usage:', response.tokenUsage);

    // Add a second prompt in the conversation
    console.log('\n--- Adding Follow-up Prompt ---');
    const followUpPrompt = await client.addPrompt({
      sessionId: session.id,
      content: 'Can you give me a simple TypeScript example?',
      metadata: {
        model: 'gpt-4',
        temperature: 0.7,
        toolsAvailable: ['code_interpreter'],
        custom: {},
      },
    });

    const followUpResponse = await client.addResponse({
      promptId: followUpPrompt.id,
      content:
        'Here\'s a simple TypeScript example:\n\n' +
        '```typescript\n' +
        'interface User {\n' +
        '  name: string;\n' +
        '  age: number;\n' +
        '}\n\n' +
        'function greet(user: User): string {\n' +
        '  return `Hello, ${user.name}!`;\n' +
        '}\n\n' +
        'const john: User = { name: "John", age: 30 };\n' +
        'console.log(greet(john));\n' +
        '```',
      tokenUsage: {
        promptTokens: 10,
        completionTokens: 65,
        totalTokens: 75,
      },
      metadata: {
        model: 'gpt-4',
        finishReason: 'stop',
        latencyMs: 1850,
        custom: {},
      },
    });
    console.log('Follow-up response created:', followUpResponse.id);

    // Query all prompts in the session
    console.log('\n--- Querying Prompts ---');
    const promptResults = await client.query({
      sessionId: session.id,
      nodeType: NodeType.PROMPT,
      limit: 10,
    });
    console.log(`Found ${promptResults.totalCount} prompts in session`);
    promptResults.nodes.forEach((node, index) => {
      console.log(`  ${index + 1}. Prompt ID: ${node.id}`);
    });

    // Query all responses in the session
    console.log('\n--- Querying Responses ---');
    const responseResults = await client.query({
      sessionId: session.id,
      nodeType: NodeType.RESPONSE,
      limit: 10,
    });
    console.log(`Found ${responseResults.totalCount} responses in session`);
    responseResults.nodes.forEach((node, index) => {
      console.log(`  ${index + 1}. Response ID: ${node.id}`);
    });

    // Get service metrics
    console.log('\n--- Service Metrics ---');
    const metrics = await client.getMetrics();
    console.log('Total Nodes:', metrics.totalNodes);
    console.log('Total Edges:', metrics.totalEdges);
    console.log('Total Sessions:', metrics.totalSessions);
    console.log('Active Sessions:', metrics.activeSessions);
    console.log('Avg Write Latency:', metrics.avgWriteLatencyMs.toFixed(2), 'ms');
    console.log('Avg Read Latency:', metrics.avgReadLatencyMs.toFixed(2), 'ms');

    // List all sessions
    console.log('\n--- Listing Sessions ---');
    const { sessions, totalCount } = await client.listSessions(5, 0);
    console.log(`Found ${totalCount} total sessions (showing first 5):`);
    sessions.forEach((s, index) => {
      console.log(`  ${index + 1}. ${s.id} (Active: ${s.isActive})`);
    });

    // Example of streaming query (demonstrates async iteration)
    console.log('\n--- Streaming Query Example ---');
    console.log('Starting stream of all nodes in session...');
    let streamCount = 0;
    await new Promise<void>((resolve, reject) => {
      client.streamQuery(
        {
          sessionId: session.id,
          limit: 100,
        },
        {
          onData: (node) => {
            streamCount++;
            console.log(`  Streamed node ${streamCount}: ${node.id} (${NodeType[node.type]})`);
          },
          onError: (error) => {
            console.error('Stream error:', error);
            reject(error);
          },
          onEnd: () => {
            console.log(`Stream completed. Received ${streamCount} nodes.`);
            resolve();
          },
        }
      );
    });

    // Clean up: Delete the session
    console.log('\n--- Cleaning Up ---');
    await client.deleteSession(session.id);
    console.log('Session deleted successfully');

    console.log('\n--- Example Completed Successfully ---');
  } catch (error) {
    console.error('\nError occurred:', error);
    throw error;
  } finally {
    // Always close the client when done
    client.close();
    console.log('\nClient connection closed');
  }
}

// Run the example
if (require.main === module) {
  main()
    .then(() => {
      console.log('\nQuickstart example finished successfully');
      process.exit(0);
    })
    .catch((error) => {
      console.error('\nQuickstart example failed:', error.message);
      process.exit(1);
    });
}

export { main };
