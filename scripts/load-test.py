"""
Load Test Scenarios for Kao Backend

This file defines load test scenarios using Locust for horizontal scaling validation.
"""

import json
import random
from locust import HttpUser, task, between, events
from locust.env import Environment
from locust.stats import stats_history


class AuthScenario(HttpUser):
    """Authentication scenarios for load testing"""
    
    wait_time = between(1, 3)  # Wait 1-3 seconds between tasks
    
    def on_start(self):
        """Initialize user session"""
        self.username = f"user{random.randint(1, 1000)}"
        self.password = "password123"
        
    @task(3)
    def login(self):
        """Login endpoint test"""
        response = self.client.post(
            "/api/auth/login",
            json={
                "username": self.username,
                "password": "admin123"
            },
            headers={"Content-Type": "application/json"}
        )
        
        if response.status_code == 200:
            data = response.json()
            if "access_token" in str(data):
                print(f"Login successful for user {self.username}")
    
    @task(1)
    def refresh_token(self):
        """Token refresh endpoint test"""
        response = self.client.post(
            "/api/auth/refresh",
            json={
                "refresh_token": "dummy_refresh_token"
            },
            headers={"Content-Type": "application/json"}
        )
        print(f"Refresh token response: {response.status_code}")
    
    @task(2)
    def get_current_user(self):
        """Get current user profile"""
        response = self.client.get(
            "/api/auth/session",
            headers={"Content-Type": "application/json"}
        )
        print(f"Get current user response: {response.status_code}")


class UserScenario(HttpUser):
    """User management scenarios"""
    
    wait_time = between(1, 2)
    
    @task(2)
    def list_users(self):
        """List users endpoint"""
        response = self.client.get(
            "/api/system/users",
            headers={"Content-Type": "application/json"}
        )
        print(f"List users response: {response.status_code}")
    
    @task(1)
    def create_user(self):
        """Create user endpoint"""
        response = self.client.post(
            "/api/system/users",
            json={
                "username": f"user{random.randint(1, 1000)}",
                "password": "password123",
                "email": f"user{random.randint(1, 1000)}@example.com",
                "displayName": f"User {random.randint(1, 1000)}",
                "status": 1
            },
            headers={"Content-Type": "application/json"}
        )
        print(f"Create user response: {response.status_code}")
    
    @task(1)
    def get_user(self):
        """Get user details endpoint"""
        user_id = "00000000-0000-0000-0000-000000000001"
        response = self.client.get(
            f"/api/system/users/{user_id}",
            headers={"Content-Type": "application/json"}
        )
        print(f"Get user response: {response.status_code}")
    
    @task(1)
    def update_user(self):
        """Update user endpoint"""
        user_id = "00000000-0000-0000-0000-000000000001"
        response = self.client.put(
            f"/api/system/users/{user_id}",
            json={
                "displayName": f"Updated User {random.randint(1, 1000)}"
            },
            headers={"Content-Type": "application/json"}
        )
        print(f"Update user response: {response.status_code}")
    
    @task(1)
    def delete_user(self):
        """Delete user endpoint (unsafe for production)"""
        user_id = "00000000-0000-0000-0000-000000000001"
        response = self.client.delete(
            f"/api/system/users/{user_id}",
            headers={"Content-Type": "application/json"}
        )
        print(f"Delete user response: {response.status_code}")
    
    @task(1)
    def reset_password(self):
        """Reset user password endpoint"""
        user_id = "00000000-0000-0000-0000-000000000001"
        response = self.client.put(
            f"/api/system/users/{user_id}/reset-password",
            json={"password": "newpassword123"},
            headers={"Content-Type": "application/json"}
        )
        print(f"Reset password response: {response.status_code}")


class DepartmentScenario(HttpUser):
    """Department management scenarios"""
    
    wait_time = between(1, 2)
    
    @task(2)
    def list_departments(self):
        """List departments endpoint"""
        response = self.client.get(
            "/api/system/departments",
            headers={"Content-Type": "application/json"}
        )
        print(f"List departments response: {response.status_code}")
    
    @task(1)
    def get_department(self):
        """Get department details"""
        dept_id = "00000000-0000-0000-0000-000000000001"
        response = self.client.get(
            f"/api/system/departments/{dept_id}",
            headers={"Content-Type": "application/json"}
        )
        print(f"Get department response: {response.status_code}")
    
    @task(1)
    def create_department(self):
        """Create department endpoint"""
        response = self.client.post(
            "/api/system/departments",
            json={
                "code": f"dept{random.randint(1, 100)}",
                "name": f"Department {random.randint(1, 100)}",
                "parentId": None,
                "leader": f"leader{random.randint(1, 100)}",
                "phone": f"1380013800{random.randint(1, 9)}",
                "email": f"dept{random.randint(1, 100)}@example.com",
                "status": "ACTIVE"
            },
            headers={"Content-Type": "application/json"}
        )
        print(f"Create department response: {response.status_code}")


class RoleScenario(HttpUser):
    """Role management scenarios"""
    
    wait_time = between(1, 2)
    
    @task(2)
    def list_roles(self):
        """List roles endpoint"""
        response = self.client.get(
            "/api/system/roles",
            headers={"Content-Type": "application/json"}
        )
        print(f"List roles response: {response.status_code}")
    
    @task(1)
    def get_role(self):
        """Get role details"""
        role_id = "00000000-0000-0000-0000-000000000001"
        response = self.client.get(
            f"/api/system/roles/{role_id}",
            headers={"Content-Type": "application/json"}
        )
        print(f"Get role response: {response.status_code}")
    
    @task(1)
    def assign_menus_to_role(self):
        """Assign menus to role endpoint"""
        role_id = "00000000-0000-0000-0000-000000000001"
        response = self.client.put(
            f"/api/system/roles/{role_id}/menus",
            json={"menuIds": ["00000000-0000-0000-0000-000000000001"]},
            headers={"Content-Type": "application/json"}
        )
        print(f"Assign menus to role response: {response.status_code}")


class MenuScenario(HttpUser):
    """Menu management scenarios"""
    
    wait_time = between(1, 2)
    
    @task(2)
    def list_menus(self):
        """List menus endpoint"""
        response = self.client.get(
            "/api/system/menus",
            headers={"Content-Type": "application/json"}
        )
        print(f"List menus response: {response.status_code}")
    
    @task(1)
    def get_menu_tree(self):
        """Get menu tree endpoint"""
        response = self.client.get(
            "/api/system/menus/tree",
            headers={"Content-Type": "application/json"}
        )
        print(f"Get menu tree response: {response.status_code}")
    
    @task(1)
    def create_menu(self):
        """Create menu endpoint"""
        response = self.client.post(
            "/api/system/menus",
            json={
                "name": f"Menu {random.randint(1, 100)}",
                "menuType": "M",
                "routePath": f"/path/{random.randint(1, 100)}",
                "permission": f"perm:{random.randint(1, 100)}",
                "sortOrder": random.randint(1, 100),
                "visible": True,
                "status": "ACTIVE"
            },
            headers={"Content-Type": "application/json"}
        )
        print(f"Create menu response: {response.status_code}")


class DictionaryScenario(HttpUser):
    """Dictionary management scenarios"""
    
    wait_time = between(1, 2)
    
    @task(2)
    def list_dictionary_types(self):
        """List dictionary types"""
        response = self.client.get(
            "/api/system/dictionary/types",
            headers={"Content-Type": "application/json"}
        )
        print(f"List dictionary types response: {response.status_code}")
    
    @task(1)
    def list_dictionary_data(self):
        """List dictionary data"""
        response = self.client.get(
            "/api/system/dictionary/data",
            headers={"Content-Type": "application/json"}
        )
        print(f"List dictionary data response: {response.status_code}")


class ConfigScenario(HttpUser):
    """Configuration management scenarios"""
    
    wait_time = between(1, 2)
    
    @task(2)
    def list_configs(self):
        """List configuration parameters"""
        response = self.client.get(
            "/api/system/config",
            headers={"Content-Type": "application/json"}
        )
        print(f"List configs response: {response.status_code}")
    
    @task(1)
    def create_config(self):
        """Create configuration parameter"""
        response = self.client.post(
            "/api/system/config",
            json={
                "configKey": f"config_{random.randint(1, 100)}",
                "configValue": f"value_{random.randint(1, 100)}",
                "configType": "Y",
                "remark": f"remark_{random.randint(1, 100)}"
            },
            headers={"Content-Type": "application/json"}
        )
        print(f"Create config response: {response.status_code}")


class MonitoringScenario(HttpUser):
    """Monitoring scenarios"""
    
    wait_time = between(1, 2)
    
    @task(2)
    def health_check(self):
        """Health check endpoint"""
        response = self.client.get(
            "/system/monitor/health",
            headers={"Content-Type": "application/json"}
        )
        print(f"Health check response: {response.status_code}")
    
    @task(1)
    def metrics(self):
        """Metrics endpoint for Prometheus"""
        response = self.client.get(
            "/metrics",
            headers={"Content-Type": "application/json"}
        )
        print(f"Metrics response: {response.status_code}")


# Performance benchmarks
PERFORMANCE_BENCHMARKS = {
    "login": {"p95": 100, "p99": 200},  # ms
    "refresh_token": {"p95": 50, "p99": 100},
    "list_users": {"p95": 50, "p99": 100},
    "create_user": {"p95": 100, "p99": 200},
    "get_user": {"p95": 30, "p99": 60},
    "list_departments": {"p95": 50, "p99": 100},
    "list_roles": {"p95": 50, "p99": 100},
    "list_menus": {"p95": 50, "p99": 100},
    "health_check": {"p95": 10, "p99": 20},
    "metrics": {"p95": 20, "p99": 50},
}


# Scalability metrics
SCALABILITY_METRICS = {
    "max_users_per_instance": 1000,
    "max_requests_per_second": 1000,
    "target_response_time_p95": 100,  # ms
    "target_failure_rate": 0.01,  # 1%
}


if __name__ == "__main__":
    # Run with: locust -f load-test.py --host http://localhost:8080
    print("Load Test Scenarios Loaded")
    print(f"Authentication Scenarios: {len(AuthScenario.tasks)}")
    print(f"User Scenarios: {len(UserScenario.tasks)}")
    print(f"Department Scenarios: {len(DepartmentScenario.tasks)}")
    print(f"Role Scenarios: {len(RoleScenario.tasks)}")
    print(f"Menu Scenarios: {len(MenuScenario.tasks)}")
    print(f"Dictionary Scenarios: {len(DictionaryScenario.tasks)}")
    print(f"Config Scenarios: {len(ConfigScenario.tasks)}")
    print(f"Monitoring Scenarios: {len(MonitoringScenario.tasks)}")
