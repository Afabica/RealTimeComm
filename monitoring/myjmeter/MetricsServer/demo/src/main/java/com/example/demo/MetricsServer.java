package com.example.demo;

import static spark.Spark.*;

import org.apache.commons.csv.CSVFormat;
import org.apache.commons.csv.CSVParser;
import org.apache.commons.csv.CSVRecord;

import java.io.FileReader;
import java.io.IOException;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.HashMap;
import java.util.Map;

public class MetricsServer {

    // Path to the JTL file (adjust this to where the file is located)
    private static final String CSV_PATH = "/jmeter/apache-jmeter-5.5/test/results.jtl"; // Use absolute path suitable for your container setup

    public static void main(String[] args) {
        // Set port and bind to all interfaces (important for Docker/Kubernetes)
        port(8000);
        ipAddress("0.0.0.0");

        // Define a route for Prometheus to scrape the metrics
        get("/metrics", (req, res) -> {
            res.type("text/plain");
            return generateMetrics();
        });

        // Start the server and log the address
        System.out.println("Server started at http://0.0.0.0:8000/metrics");
    }

    private static String generateMetrics() {
        int totalRequests = 0;
        int totalFailures = 0;
        long totalElapsed = 0;
        Map<String, Integer> labelCounts = new HashMap<>();

        // Read and parse the JTL file
        try (FileReader reader = new FileReader(Paths.get(CSV_PATH).toFile());
             CSVParser csvParser = new CSVParser(reader, CSVFormat.DEFAULT.withFirstRecordAsHeader())) {

            for (CSVRecord record : csvParser) {
                totalRequests++;
                totalElapsed += Long.parseLong(record.get("elapsed"));

                // Count failures based on success field
                String success = record.get("success");
                if (!"true".equalsIgnoreCase(success)) {
                    totalFailures++;
                }

                // Collect request label counts
                String label = record.get("label").replace(" ", "_").toLowerCase();
                labelCounts.put(label, labelCounts.getOrDefault(label, 0) + 1);
            }
        } catch (IOException e) {
            e.printStackTrace();
            return "# Error reading CSV file\n";
        }

        // Handle case where no data is available
        if (totalRequests == 0) {
            return "# No data available\n";
        }

        // Calculate average response time and error rate
        double avgResponseTimeSec = totalElapsed / (double) totalRequests / 1000;
        double errorRate = totalFailures / (double) totalRequests;

        // Build the Prometheus-formatted metrics
        StringBuilder metrics = new StringBuilder();
        metrics.append("jmeter_total_requests ").append(totalRequests).append("\n");
        metrics.append("jmeter_total_failures ").append(totalFailures).append("\n");
        metrics.append("jmeter_avg_response_time_seconds ").append(String.format("%.3f", avgResponseTimeSec)).append("\n");
        metrics.append("jmeter_error_rate ").append(String.format("%.3f", errorRate)).append("\n");

        // Add label-specific metrics
        for (Map.Entry<String, Integer> entry : labelCounts.entrySet()) {
            metrics.append("jmeter_label_requests{label=\"").append(entry.getKey()).append("\"} ")
                    .append(entry.getValue()).append("\n");
        }

        return metrics.toString();
    }
}

