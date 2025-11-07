/**
 * Proto loader for dynamic gRPC client generation
 * This file provides a dynamic loader for the protobuf definitions
 */

import * as grpc from '@grpc/grpc-js';
import * as protoLoader from '@grpc/proto-loader';
import * as path from 'path';

const PROTO_PATH = path.join(__dirname, '../../proto/memory_graph.proto');

const packageDefinition = protoLoader.loadSync(PROTO_PATH, {
  keepCase: true,
  longs: String,
  enums: String,
  defaults: true,
  oneofs: true,
  includeDirs: [path.join(__dirname, '../../proto')],
});

export const proto = grpc.loadPackageDefinition(packageDefinition) as any;
export const MemoryGraphService = proto.llm.memory.graph.v1.MemoryGraphService;
