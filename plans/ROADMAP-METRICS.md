# LLM-Memory-Graph: Success Metrics & Measurement Framework

## Overview

This document defines measurable success criteria for each phase of the LLM-Memory-Graph roadmap, including specific KPIs, measurement methodologies, and instrumentation requirements.

---

## Phase 1: MVP Success Metrics

### 1.1 Performance Metrics

#### Graph Operation Latency
**Target:** p95 < 100ms, p99 < 200ms

**Measurement:**
```python
# Instrumentation
import time
from prometheus_client import Histogram

graph_operation_duration = Histogram(
    'graph_operation_duration_seconds',
    'Time spent on graph operations',
    ['operation_type']  # add_node, query, traverse, etc.
)

@graph_operation_duration.labels('add_node').time()
def add_node(node: MemoryNode):
    # implementation
    pass
```

**Dashboard Query:**
```promql
# Prometheus query
histogram_quantile(0.95,
  rate(graph_operation_duration_seconds_bucket[5m])
)
```

**Success Criteria:**
- 95% of graph operations complete in < 100ms
- No operations exceed 1 second
- Linear scaling up to 10K nodes

---

#### Context Retrieval Accuracy
**Target:** > 70% relevance (manual evaluation)

**Measurement Methodology:**
1. **Create golden test set:**
   - 100 diverse queries
   - Human-annotated relevant results (top-10)
   - Cover different query types (factual, temporal, relational)

2. **Evaluation metrics:**
   - **Precision@K:** Fraction of retrieved results that are relevant
   - **Recall@K:** Fraction of relevant results that are retrieved
   - **MRR (Mean Reciprocal Rank):** Average of 1/rank of first relevant result
   - **NDCG@K:** Normalized Discounted Cumulative Gain

3. **Automated evaluation:**
```python
# evaluation/retrieval_eval.py
def evaluate_retrieval(test_set: List[TestCase]) -> EvalMetrics:
    results = []
    for case in test_set:
        retrieved = retrieval_engine.search(case.query, k=10)
        results.append({
            'precision': precision_at_k(retrieved, case.relevant, k=10),
            'recall': recall_at_k(retrieved, case.relevant, k=10),
            'mrr': mean_reciprocal_rank(retrieved, case.relevant),
            'ndcg': ndcg_at_k(retrieved, case.relevant, k=10)
        })

    return EvalMetrics(
        avg_precision=mean([r['precision'] for r in results]),
        avg_recall=mean([r['recall'] for r in results]),
        avg_mrr=mean([r['mrr'] for r in results]),
        avg_ndcg=mean([r['ndcg'] for r in results])
    )
```

**Success Criteria:**
- Precision@10 > 0.70
- Recall@10 > 0.60
- MRR > 0.75

---

#### System Uptime
**Target:** > 95%

**Measurement:**
```python
# monitoring/uptime.py
from prometheus_client import Gauge

uptime_gauge = Gauge('system_uptime', 'System uptime in seconds')

# Report uptime every minute
def report_uptime():
    while True:
        uptime_gauge.set(time.time() - START_TIME)
        time.sleep(60)
```

**Calculation:**
```
Uptime % = (Total Time - Downtime) / Total Time * 100

Where Downtime = Sum of all incidents where health check fails
```

**Success Criteria:**
- Monthly uptime > 95% (allows ~36 hours downtime/month)
- No single incident > 4 hours
- MTTR (Mean Time To Recovery) < 30 minutes

---

### 1.2 Adoption Metrics

#### Developer Onboarding Time
**Target:** < 2 hours (from zero to first API call)

**Measurement:**
1. **Time tracking:**
   - Start: Developer lands on documentation
   - End: First successful API call (logged via telemetry)

2. **Instrumentation:**
```python
# Track signup to first API call
from datetime import datetime

class OnboardingTracker:
    def track_signup(self, user_id: str):
        redis.set(f'signup:{user_id}', datetime.now().isoformat())

    def track_first_api_call(self, user_id: str):
        signup_time = redis.get(f'signup:{user_id}')
        if signup_time:
            delta = datetime.now() - datetime.fromisoformat(signup_time)
            metrics.histogram('onboarding_duration_hours', delta.total_seconds() / 3600)
```

3. **Qualitative feedback:**
   - Post-onboarding survey (NPS)
   - "Was our documentation clear?" (1-5 scale)
   - "Did you encounter blockers?" (open-ended)

**Success Criteria:**
- Median onboarding time < 1 hour
- 90th percentile < 2 hours
- NPS > 50
- Documentation clarity rating > 4.0/5.0

---

#### API Response Time
**Target:** p95 < 500ms

**Measurement:**
```python
# middleware/timing.py
from fastapi import Request
import time

@app.middleware("http")
async def track_request_time(request: Request, call_next):
    start_time = time.time()
    response = await call_next(request)
    duration = time.time() - start_time

    # Record metric
    api_latency.labels(
        method=request.method,
        endpoint=request.url.path,
        status_code=response.status_code
    ).observe(duration)

    return response
```

**Alerting Rules:**
```yaml
# alertmanager/rules.yml
groups:
  - name: api_performance
    rules:
      - alert: HighAPILatency
        expr: |
          histogram_quantile(0.95,
            rate(api_latency_bucket[5m])
          ) > 0.5
        for: 5m
        annotations:
          summary: "API p95 latency > 500ms for 5 minutes"
```

**Success Criteria:**
- p50 < 200ms
- p95 < 500ms
- p99 < 1000ms
- Error rate < 0.1%

---

#### Documentation Coverage
**Target:** > 80% of core features

**Measurement:**
1. **Feature inventory:**
   - List all MVP features (from roadmap)
   - Total: ~20 core features

2. **Documentation checklist:**
   - [ ] Feature overview (what it does)
   - [ ] Code example (working snippet)
   - [ ] API reference (parameters, returns)
   - [ ] Common pitfalls (gotchas)

3. **Automated validation:**
```python
# docs/validate_coverage.py
def validate_docs_coverage():
    features = load_feature_list()  # From roadmap
    documented = find_documented_features()  # Parse docs

    coverage = len(documented) / len(features) * 100
    missing = set(features) - set(documented)

    print(f"Documentation coverage: {coverage:.1f}%")
    if missing:
        print(f"Missing docs for: {', '.join(missing)}")

    assert coverage >= 80, f"Coverage {coverage:.1f}% < 80%"
```

**Success Criteria:**
- 80% of features have complete documentation
- All API endpoints have OpenAPI descriptions
- At least 10 code examples in docs
- Getting started guide < 1500 words

---

### 1.3 Technical Metrics

#### Graph Storage Capacity
**Target:** 10K nodes, 50K edges

**Measurement:**
```python
# monitoring/graph_stats.py
from prometheus_client import Gauge

graph_nodes_total = Gauge('graph_nodes_total', 'Total nodes in graph')
graph_edges_total = Gauge('graph_edges_total', 'Total edges in graph')

def update_graph_stats():
    """Run every 5 minutes"""
    while True:
        graph_nodes_total.set(graph.node_count())
        graph_edges_total.set(graph.edge_count())
        time.sleep(300)
```

**Load Testing:**
```python
# tests/load/test_capacity.py
def test_graph_capacity():
    """Validate system handles 10K nodes, 50K edges"""
    # Generate synthetic graph
    for i in range(10000):
        graph.add_node(MemoryNode(id=f'node_{i}', type='entity'))

    for i in range(50000):
        source = f'node_{random.randint(0, 9999)}'
        target = f'node_{random.randint(0, 9999)}'
        graph.add_edge(MemoryEdge(source=source, target=target))

    # Verify operations still performant
    start = time.time()
    results = graph.query(QueryPattern(type='entity', limit=100))
    duration = time.time() - start

    assert duration < 0.1, f"Query too slow: {duration}s"
    assert len(results) == 100
```

**Success Criteria:**
- System handles 10K nodes without degradation
- Query latency remains < 100ms p95
- Memory usage < 2GB
- Can grow to 50K nodes (tested)

---

#### Concurrent Sessions Supported
**Target:** 50+ simultaneous sessions

**Measurement:**
```python
# tests/load/test_concurrency.py
import asyncio

async def simulate_session():
    """Simulate a user session"""
    session_id = str(uuid.uuid4())
    for _ in range(10):
        await api_client.add_message(session_id, "test message")
        await api_client.search(session_id, "test query")
        await asyncio.sleep(random.uniform(0.1, 1.0))

async def test_concurrent_sessions():
    """Test 50 concurrent sessions"""
    tasks = [simulate_session() for _ in range(50)]
    results = await asyncio.gather(*tasks, return_exceptions=True)

    errors = [r for r in results if isinstance(r, Exception)]
    assert len(errors) == 0, f"{len(errors)} sessions failed"
```

**Monitoring:**
```promql
# Active sessions (last 5 minutes)
count(
  count by (session_id) (
    rate(api_requests_total[5m])
  )
)
```

**Success Criteria:**
- 50 concurrent sessions with < 1% error rate
- No performance degradation with concurrency
- Linear scaling up to 100 sessions (stretch goal)

---

#### Memory Footprint
**Target:** < 2GB for typical workload

**Measurement:**
```python
# monitoring/memory.py
import psutil
from prometheus_client import Gauge

memory_usage_bytes = Gauge('process_memory_usage_bytes', 'Memory usage in bytes')

def track_memory():
    while True:
        process = psutil.Process()
        memory_usage_bytes.set(process.memory_info().rss)
        time.sleep(60)
```

**Typical Workload Definition:**
- 5,000 nodes (messages, entities, topics)
- 20,000 edges
- 10 active sessions
- 100 requests/minute

**Load Test:**
```python
# tests/load/test_memory.py
def test_memory_footprint():
    # Setup typical workload
    setup_typical_workload()

    # Measure steady-state memory
    process = psutil.Process()
    memory_mb = process.memory_info().rss / 1024 / 1024

    assert memory_mb < 2048, f"Memory usage {memory_mb:.0f}MB > 2048MB"
```

**Success Criteria:**
- Steady-state memory < 2GB
- No memory leaks (24-hour stability test)
- GC pauses < 100ms

---

## Phase 2: Beta Success Metrics

### 2.1 Performance Metrics

#### Graph Query Latency (Large Scale)
**Target:** p95 < 200ms for 100K nodes

**Measurement:**
Same instrumentation as MVP, but test at scale:

```python
# tests/load/test_large_scale.py
def test_large_scale_performance():
    # Generate 100K node graph
    generate_large_graph(nodes=100_000, edges=500_000)

    # Run diverse queries
    queries = [
        QueryPattern(type='entity', limit=10),
        QueryPattern(path_length=3, start='node_1000'),
        QueryPattern(community_detection=True),
    ]

    latencies = []
    for query in queries * 100:  # 300 total queries
        start = time.time()
        graph.query(query)
        latencies.append(time.time() - start)

    p95 = np.percentile(latencies, 95)
    assert p95 < 0.2, f"p95 latency {p95:.3f}s > 200ms"
```

**Success Criteria:**
- p95 < 200ms at 100K nodes
- p99 < 500ms at 100K nodes
- Can scale to 1M nodes (tested in staging)

---

#### Entity Extraction F1 Score
**Target:** > 0.80

**Measurement:**
1. **Create evaluation dataset:**
   - 500 diverse text samples
   - Human-annotated entities (gold standard)
   - Cover domains: general, technical, medical

2. **Compute F1:**
```python
# evaluation/ner_eval.py
def evaluate_entity_extraction(test_set: List[TextSample]) -> F1Score:
    results = []
    for sample in test_set:
        predicted = ner.extract_entities(sample.text)
        gold = sample.entities

        tp = len(predicted & gold)  # True positives
        fp = len(predicted - gold)  # False positives
        fn = len(gold - predicted)  # False negatives

        precision = tp / (tp + fp) if (tp + fp) > 0 else 0
        recall = tp / (tp + fn) if (tp + fn) > 0 else 0
        f1 = 2 * precision * recall / (precision + recall) if (precision + recall) > 0 else 0

        results.append(f1)

    return {
        'mean_f1': np.mean(results),
        'std_f1': np.std(results),
        'min_f1': np.min(results),
        'max_f1': np.max(results)
    }
```

**Continuous Monitoring:**
- Run evaluation weekly on held-out test set
- Track F1 over time (detect regressions)
- Break down by entity type (person, org, location, etc.)

**Success Criteria:**
- Overall F1 > 0.80
- Per-type F1 > 0.75 for all major types
- No regression from previous week

---

#### Context Relevance (LLM-as-Judge)
**Target:** > 80%

**Measurement:**
1. **Generate query-context pairs:**
   - User query → Retrieved context
   - 100 diverse examples

2. **LLM judging prompt:**
```python
# evaluation/llm_judge.py
JUDGE_PROMPT = """
You are evaluating the relevance of retrieved context for a given query.

Query: {query}

Retrieved Context:
{context}

Rate the relevance on a scale of 1-5:
1 - Completely irrelevant
2 - Somewhat relevant
3 - Moderately relevant
4 - Very relevant
5 - Perfectly relevant

Provide your rating and a brief explanation.

Rating: [1-5]
Explanation: [2-3 sentences]
"""

def judge_relevance(query: str, context: str) -> float:
    prompt = JUDGE_PROMPT.format(query=query, context=context)
    response = llm.complete(prompt)

    # Parse rating from response
    rating = parse_rating(response)  # Extract 1-5
    return rating / 5.0  # Normalize to 0-1
```

3. **Aggregate scores:**
```python
def evaluate_context_relevance(test_set: List[QueryContextPair]) -> float:
    scores = [judge_relevance(pair.query, pair.context) for pair in test_set]
    return np.mean(scores)
```

**Success Criteria:**
- Average relevance > 0.80 (4/5 rating)
- No queries with relevance < 0.40
- Human spot-check agrees with LLM judge 90%+ of the time

---

### 2.2 Adoption Metrics

#### Beta User Retention (30-Day)
**Target:** > 60%

**Measurement:**
```sql
-- Define active user: made at least 1 API call in the week
WITH user_cohorts AS (
  SELECT
    user_id,
    DATE_TRUNC('week', signup_date) as cohort_week
  FROM users
  WHERE signup_date >= NOW() - INTERVAL '30 days'
),
weekly_activity AS (
  SELECT
    user_id,
    DATE_TRUNC('week', api_call_timestamp) as activity_week
  FROM api_logs
  WHERE api_call_timestamp >= NOW() - INTERVAL '30 days'
  GROUP BY user_id, DATE_TRUNC('week', api_call_timestamp)
)
SELECT
  cohort_week,
  COUNT(DISTINCT uc.user_id) as cohort_size,
  COUNT(DISTINCT wa4.user_id) as retained_week4,
  COUNT(DISTINCT wa4.user_id)::float / COUNT(DISTINCT uc.user_id) as retention_rate
FROM user_cohorts uc
LEFT JOIN weekly_activity wa4
  ON uc.user_id = wa4.user_id
  AND wa4.activity_week = uc.cohort_week + INTERVAL '4 weeks'
GROUP BY cohort_week
ORDER BY cohort_week DESC;
```

**Visualization:**
```python
# analytics/retention.py
def plot_retention_curves():
    """Plot cohort retention curves"""
    cohorts = get_cohort_data()

    for cohort in cohorts:
        weeks = range(1, 5)  # 4 weeks
        retention = [cohort.retention_week(w) for w in weeks]
        plt.plot(weeks, retention, label=cohort.name)

    plt.xlabel('Weeks Since Signup')
    plt.ylabel('Retention Rate')
    plt.title('Beta User Retention')
    plt.legend()
    plt.savefig('retention_curves.png')
```

**Success Criteria:**
- Week 1 retention > 80%
- Week 2 retention > 70%
- Week 4 retention > 60%
- Month-over-month improvement

---

#### Feature Adoption Rate
**Target:** > 40% try visualization

**Measurement:**
```python
# Track feature usage
from prometheus_client import Counter

feature_usage_total = Counter(
    'feature_usage_total',
    'Number of times each feature is used',
    ['feature_name', 'user_id']
)

# Instrument features
@app.get("/api/v1/visualization/{session_id}")
async def get_visualization(session_id: str, user_id: str = Depends(get_user)):
    feature_usage_total.labels('visualization', user_id).inc()
    # ... implementation
```

**Calculation:**
```sql
-- Feature adoption rate
SELECT
  feature_name,
  COUNT(DISTINCT user_id) as users_tried,
  COUNT(DISTINCT user_id)::float / (SELECT COUNT(*) FROM users) as adoption_rate
FROM feature_usage_logs
WHERE created_at >= NOW() - INTERVAL '30 days'
GROUP BY feature_name
ORDER BY adoption_rate DESC;
```

**Success Criteria:**
- Visualization: 40%+ adoption
- Advanced queries: 25%+ adoption
- Multi-hop reasoning: 15%+ adoption
- Community detection: 10%+ adoption

---

#### Community Contributions
**Target:** 10+ PRs from external developers

**Measurement:**
```bash
# Count external contributions
gh pr list --state all --json author,authorAssociation \
  --jq '[.[] | select(.authorAssociation == "FIRST_TIME_CONTRIBUTOR" or .authorAssociation == "CONTRIBUTOR")] | length'
```

**Tracking:**
- Tag PRs with labels: `community`, `good-first-issue`, `help-wanted`
- Track time-to-merge for community PRs
- Monitor contributor retention (do they submit 2nd PR?)

**Success Criteria:**
- 10+ PRs merged from external contributors
- 5+ repeat contributors (2+ PRs each)
- Median time-to-review < 48 hours
- 80%+ PR acceptance rate (well-scoped issues)

---

### 2.3 Scalability Metrics

#### Graph Capacity
**Target:** 1M+ nodes, 5M+ edges

**Measurement:**
Same as MVP, but at scale:

```python
# tests/load/test_massive_scale.py
@pytest.mark.slow
def test_million_node_capacity():
    """This test takes ~1 hour to run"""
    # Generate 1M nodes
    for i in range(1_000_000):
        if i % 10000 == 0:
            print(f"Progress: {i/10000:.0f}% ({i} nodes)")
        graph.add_node(MemoryNode(id=f'node_{i}', type='entity'))

    # Generate 5M edges
    for i in range(5_000_000):
        source = f'node_{random.randint(0, 999_999)}'
        target = f'node_{random.randint(0, 999_999)}'
        graph.add_edge(MemoryEdge(source=source, target=target))

    # Verify queries still work
    results = graph.query(QueryPattern(type='entity', limit=100))
    assert len(results) == 100

    # Verify latency acceptable
    start = time.time()
    graph.query(QueryPattern(path_length=3, start='node_1000'))
    duration = time.time() - start
    assert duration < 0.5, f"Query too slow at scale: {duration}s"
```

**Success Criteria:**
- System stable at 1M nodes, 5M edges
- Query latency p95 < 500ms at scale
- Memory usage < 10GB
- No data corruption after 1M writes

---

#### Ingestion Throughput
**Target:** 100+ messages/sec

**Measurement:**
```python
# tests/load/test_throughput.py
import time

def test_ingestion_throughput():
    """Measure sustained ingestion rate"""
    num_messages = 10_000
    session_id = "throughput_test"

    start = time.time()
    for i in range(num_messages):
        api_client.add_message(
            session_id=session_id,
            message=f"Test message {i}",
            speaker="USER"
        )
    duration = time.time() - start

    throughput = num_messages / duration
    print(f"Throughput: {throughput:.1f} messages/sec")

    assert throughput >= 100, f"Throughput {throughput:.1f} < 100 msg/sec"
```

**Monitoring:**
```promql
# Real-time ingestion rate
rate(messages_ingested_total[1m])
```

**Success Criteria:**
- Sustained throughput > 100 msg/sec
- Burst capacity > 500 msg/sec (1 minute)
- No message loss under load
- Queue depth < 100 messages

---

## Phase 3: v1.0 Success Metrics

### 3.1 Reliability Metrics

#### System Uptime (SLA)
**Target:** 99.9% (43 min downtime/month)

**Measurement:**
```python
# monitoring/sla.py
class SLATracker:
    def __init__(self):
        self.start_time = datetime.now()
        self.downtime_intervals = []

    def record_downtime(self, start: datetime, end: datetime):
        self.downtime_intervals.append((start, end))

    def calculate_uptime(self) -> float:
        total_time = (datetime.now() - self.start_time).total_seconds()
        total_downtime = sum(
            (end - start).total_seconds()
            for start, end in self.downtime_intervals
        )
        return (total_time - total_downtime) / total_time * 100

    def meets_sla(self) -> bool:
        return self.calculate_uptime() >= 99.9
```

**Alerting:**
```yaml
# Alert if downtime will breach SLA
- alert: SLABudgetLow
  expr: |
    (1 - avg_over_time(up[30d])) * 100 > 0.08  # 80% of budget used
  annotations:
    summary: "SLA budget 80% consumed (monthly)"
```

**Success Criteria:**
- Monthly uptime ≥ 99.9%
- No single incident > 1 hour
- Planned maintenance < 10 minutes/month
- SLA credits issued < $1K/month

---

#### MTTR (Mean Time To Recovery)
**Target:** < 15 minutes

**Measurement:**
```python
# incidents/tracker.py
class IncidentTracker:
    def record_incident(
        self,
        detected_at: datetime,
        resolved_at: datetime,
        severity: str
    ):
        mttr = (resolved_at - detected_at).total_seconds() / 60

        incident_mttr.labels(severity=severity).observe(mttr)

        # Store for reporting
        db.insert_incident({
            'detected_at': detected_at,
            'resolved_at': resolved_at,
            'mttr_minutes': mttr,
            'severity': severity
        })
```

**Dashboard:**
```sql
-- MTTR by month
SELECT
  DATE_TRUNC('month', detected_at) as month,
  AVG(mttr_minutes) as avg_mttr,
  MAX(mttr_minutes) as max_mttr,
  COUNT(*) as incident_count
FROM incidents
WHERE severity IN ('high', 'critical')
GROUP BY DATE_TRUNC('month', detected_at)
ORDER BY month DESC;
```

**Success Criteria:**
- Average MTTR < 15 minutes
- p95 MTTR < 30 minutes
- Zero incidents with MTTR > 4 hours
- Improving trend month-over-month

---

### 3.2 Quality Metrics

#### Zero-Day Security Vulnerabilities
**Target:** Zero

**Measurement:**
1. **Automated scanning:**
```yaml
# .github/workflows/security.yml
name: Security Scan
on: [push, pull_request]
jobs:
  security:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Run Snyk
        uses: snyk/actions/python@master
        with:
          command: test
          args: --severity-threshold=high
      - name: Run Bandit
        run: bandit -r src/ -ll
      - name: Run Safety
        run: safety check --json
```

2. **Dependency tracking:**
```bash
# Monitor for new CVEs in dependencies
dependabot security updates --auto-merge
```

3. **Penetration testing:**
- Quarterly external pen tests
- Bug bounty program (HackerOne/Bugcrowd)

**Success Criteria:**
- Zero high/critical vulnerabilities in production
- All dependencies up-to-date (< 30 days old)
- Bug bounty program launched
- Security response SLA < 24 hours (critical)

---

#### Test Coverage
**Target:** > 90%

**Measurement:**
```bash
# pytest with coverage
pytest --cov=src --cov-report=html --cov-report=term --cov-fail-under=90
```

**Coverage by module:**
```
Name                      Stmts   Miss  Cover
-------------------------------------------
src/core/graph.py           250      5    98%
src/ingestion/pipeline.py   180     12    93%
src/retrieval/engine.py     220     25    89%  ← Below target
src/api/routes.py           150      3    98%
-------------------------------------------
TOTAL                      1200    100    92%
```

**Success Criteria:**
- Overall coverage ≥ 90%
- No module < 80% coverage
- Critical paths (auth, payment) ≥ 95%
- Integration test coverage ≥ 70%

---

### 3.3 Adoption Metrics

#### Production Deployments
**Target:** 100+ organizations

**Measurement:**
```python
# Track deployments via telemetry (opt-in)
class TelemetryClient:
    def report_deployment(self, org_id: str, deployment_type: str):
        """Anonymous deployment tracking"""
        requests.post('https://telemetry.memorygraph.io/deployment', json={
            'org_id': hash_org_id(org_id),  # Anonymized
            'deployment_type': deployment_type,  # k8s, docker, vm, etc.
            'version': VERSION,
            'timestamp': datetime.now().isoformat()
        })
```

**Dashboard:**
```sql
-- Deployments by type
SELECT
  deployment_type,
  COUNT(DISTINCT org_id) as org_count,
  COUNT(*) as total_deployments
FROM telemetry_deployments
WHERE first_seen_at >= NOW() - INTERVAL '90 days'
GROUP BY deployment_type
ORDER BY org_count DESC;
```

**Success Criteria:**
- 100+ unique organizations
- 50+ production deployments (not just trials)
- Deployment diversity: 30% k8s, 30% docker, 20% cloud, 20% other

---

#### GitHub Stars
**Target:** 5,000+

**Measurement:**
```bash
# Track GitHub stars over time
gh api repos/{owner}/{repo} --jq '.stargazers_count'
```

**Growth Tracking:**
```python
# scripts/track_github_metrics.py
import requests
from datetime import datetime

def track_github_stars():
    response = requests.get('https://api.github.com/repos/owner/repo')
    stars = response.json()['stargazers_count']

    db.insert_metric({
        'metric': 'github_stars',
        'value': stars,
        'timestamp': datetime.now()
    })
```

**Success Criteria:**
- 5,000+ stars at v1.0 launch
- Growth rate > 100 stars/month
- Star-to-fork ratio > 10:1 (engagement)

---

#### Package Downloads
**Target:** 50K+/month

**Measurement:**
```bash
# PyPI downloads (via pypistats)
pypistats recent llm-memory-graph --period month

# NPM downloads
npm-stat llm-memory-graph --start-date 2025-10-01
```

**Tracking:**
```sql
-- Store in database for trending
INSERT INTO package_stats (package, source, downloads, month)
VALUES ('llm-memory-graph', 'pypi', 52341, '2025-11-01');
```

**Success Criteria:**
- PyPI: 30K+ downloads/month
- NPM: 20K+ downloads/month
- Growing month-over-month
- 60% of downloads from organic search

---

### 3.4 Business Metrics

#### Free-to-Paid Conversion
**Target:** > 5%

**Measurement:**
```sql
-- Conversion funnel
WITH funnel AS (
  SELECT
    COUNT(DISTINCT user_id) as free_signups,
    COUNT(DISTINCT CASE WHEN plan != 'free' THEN user_id END) as paid_users,
    COUNT(DISTINCT CASE WHEN plan != 'free' THEN user_id END)::float /
      COUNT(DISTINCT user_id) as conversion_rate
  FROM users
  WHERE signup_date >= NOW() - INTERVAL '90 days'
)
SELECT * FROM funnel;
```

**Cohort Analysis:**
```sql
-- Time to conversion by cohort
SELECT
  DATE_TRUNC('month', signup_date) as cohort,
  AVG(EXTRACT(EPOCH FROM (first_payment_date - signup_date)) / 86400) as avg_days_to_convert,
  COUNT(*) as converted_users
FROM users
WHERE plan != 'free'
GROUP BY DATE_TRUNC('month', signup_date)
ORDER BY cohort DESC;
```

**Success Criteria:**
- Overall conversion rate ≥ 5%
- Time to conversion < 30 days (median)
- Increasing conversion rate month-over-month

---

#### Monthly Recurring Revenue (MRR)
**Target:** $50K+

**Measurement:**
```sql
-- Current MRR
SELECT
  SUM(CASE
    WHEN plan = 'pro' THEN 49
    WHEN plan = 'enterprise' THEN 499
    ELSE 0
  END) as mrr
FROM users
WHERE plan != 'free' AND status = 'active';
```

**Tracking:**
```python
# scripts/calculate_mrr.py
def calculate_mrr():
    """Calculate MRR and related metrics"""
    active_subs = get_active_subscriptions()

    mrr = sum(sub.monthly_value for sub in active_subs)
    new_mrr = sum(sub.monthly_value for sub in active_subs if sub.is_new_this_month)
    churned_mrr = sum(sub.monthly_value for sub in get_churned_this_month())

    return {
        'mrr': mrr,
        'new_mrr': new_mrr,
        'churned_mrr': churned_mrr,
        'net_new_mrr': new_mrr - churned_mrr,
        'growth_rate': (new_mrr - churned_mrr) / (mrr - new_mrr) * 100 if mrr > new_mrr else 0
    }
```

**Success Criteria:**
- Total MRR ≥ $50K
- Month-over-month growth ≥ 10%
- Net churn rate < 5%
- LTV/CAC ratio > 3:1

---

#### Customer Satisfaction (CSAT)
**Target:** > 4.5/5

**Measurement:**
```python
# Post-interaction survey
class CSATSurvey:
    QUESTION = "How satisfied are you with LLM-Memory-Graph?"

    def send_survey(self, user_id: str):
        """Send 30 days after signup"""
        send_email(user_id, template='csat_survey', data={
            'survey_link': f'https://survey.example.com/{user_id}'
        })

    def record_response(self, user_id: str, rating: int):
        """Rating: 1-5"""
        db.insert_response({
            'user_id': user_id,
            'rating': rating,
            'timestamp': datetime.now()
        })

        csat_rating.observe(rating)
```

**Calculation:**
```sql
-- CSAT score (last 90 days)
SELECT
  AVG(rating) as csat_score,
  COUNT(*) as responses,
  COUNT(*) FILTER (WHERE rating >= 4) * 100.0 / COUNT(*) as satisfied_pct
FROM csat_responses
WHERE created_at >= NOW() - INTERVAL '90 days';
```

**Success Criteria:**
- Average rating ≥ 4.5/5
- Response rate ≥ 30%
- "Satisfied" (4-5 rating) ≥ 85%
- No ratings < 3 without follow-up

---

## Measurement Infrastructure

### Dashboard Setup

#### Grafana Dashboards
```yaml
# dashboards/overview.json
{
  "dashboard": {
    "title": "LLM-Memory-Graph Overview",
    "panels": [
      {
        "title": "API Latency (p95)",
        "targets": [
          {
            "expr": "histogram_quantile(0.95, rate(api_latency_bucket[5m]))"
          }
        ]
      },
      {
        "title": "Active Users (24h)",
        "targets": [
          {
            "expr": "count(count by (user_id) (rate(api_requests_total[24h])))"
          }
        ]
      },
      {
        "title": "Error Rate",
        "targets": [
          {
            "expr": "rate(api_requests_total{status_code=~\"5..\"}[5m])"
          }
        ]
      }
    ]
  }
}
```

#### Weekly Metrics Report
```python
# scripts/weekly_report.py
def generate_weekly_report():
    """Auto-generated weekly metrics report"""
    report = {
        'week_ending': datetime.now(),
        'performance': {
            'api_latency_p95': get_metric('api_latency_p95'),
            'uptime': get_metric('uptime_pct'),
            'error_rate': get_metric('error_rate')
        },
        'adoption': {
            'signups': get_metric('signups_this_week'),
            'active_users': get_metric('active_users'),
            'retention': get_metric('retention_30d')
        },
        'business': {
            'mrr': get_metric('mrr'),
            'new_customers': get_metric('new_customers_this_week'),
            'churn_rate': get_metric('churn_rate')
        }
    }

    # Send to stakeholders
    send_email(recipients=STAKEHOLDERS, template='weekly_report', data=report)
```

---

## Conclusion

This metrics framework provides:

1. **Clear targets** for each phase
2. **Measurement methodologies** with code examples
3. **Success criteria** for objective evaluation
4. **Monitoring infrastructure** requirements
5. **Reporting cadence** (weekly, monthly, quarterly)

**Key Principles:**
- **Instrument early:** Add metrics before launch, not after
- **Automate collection:** Manual reporting doesn't scale
- **Visualize trends:** Dashboards for real-time monitoring
- **Act on data:** Metrics should drive decisions
- **Iterate on metrics:** Refine what you measure over time

**Next Steps:**
1. Set up Prometheus + Grafana stack
2. Implement instrumentation in code
3. Create initial dashboards
4. Establish weekly review cadence
5. Define alert thresholds and on-call runbooks
