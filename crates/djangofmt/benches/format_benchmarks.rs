use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use djangofmt::args::Profile;
use djangofmt::commands::format::{FormatterConfig, format_text};
use std::borrow::Cow;

// Sample Django template for benchmarking
const SIMPLE_TEMPLATE: &str = r#"
<!DOCTYPE html>
<html>
<head>
    <title>{{ title }}</title>
    <style>
        body { margin: 0; padding: 20px; }
        .container { max-width: 1200px; margin: 0 auto; }
    </style>
</head>
<body>
    <div class="container">
        <h1>{{ heading }}</h1>
        {% if user.is_authenticated %}
            <p>Welcome, {{ user.username }}!</p>
        {% else %}
            <p>Please log in.</p>
        {% endif %}
        <ul>
        {% for item in items %}
            <li>{{ item.name }} - ${{ item.price }}</li>
        {% endfor %}
        </ul>
    </div>
</body>
</html>
"#;

const COMPLEX_TEMPLATE: &str = r#"
{% extends "base.html" %}
{% load static i18n %}

{% block title %}{{ block.super }} - {{ page_title }}{% endblock %}

{% block extra_css %}
<style>
    .product-grid{display:grid;grid-template-columns:repeat(auto-fill,minmax(250px,1fr));gap:20px;padding:20px;}
    .product-card{border:1px solid #ddd;border-radius:8px;padding:15px;transition:transform 0.2s;}
    .product-card:hover{transform:translateY(-5px);box-shadow:0 4px 8px rgba(0,0,0,0.1);}
</style>
{% endblock %}

{% block content %}
<div class="container">
    <h1>{% trans "Product Catalog" %}</h1>
    
    {% if messages %}
        <div class="messages">
        {% for message in messages %}
            <div class="alert alert-{{ message.tags }}">{{ message }}</div>
        {% endfor %}
        </div>
    {% endif %}

    <div class="product-grid">
    {% for product in products %}
        <div class="product-card" data-id="{{ product.id }}">
            <img src="{% if product.image %}{{ product.image.url }}{% else %}{% static 'images/placeholder.jpg' %}{% endif %}" alt="{{ product.name }}">
            <h3>{{ product.name }}</h3>
            <p class="description">{{ product.description|truncatewords:20 }}</p>
            <div class="pricing">
                {% if product.on_sale %}
                    <span class="original-price">${{ product.original_price }}</span>
                    <span class="sale-price">${{ product.sale_price }}</span>
                {% else %}
                    <span class="price">${{ product.price }}</span>
                {% endif %}
            </div>
            <button class="btn btn-primary" data-product="{{ product.id }}">
                {% trans "Add to Cart" %}
            </button>
        </div>
    {% empty %}
        <p>{% trans "No products available." %}</p>
    {% endfor %}
    </div>

    {% if is_paginated %}
        <nav class="pagination">
            {% if page_obj.has_previous %}
                <a href="?page=1">&laquo; {% trans "First" %}</a>
                <a href="?page={{ page_obj.previous_page_number }}">{% trans "Previous" %}</a>
            {% endif %}
            <span class="current">
                {% trans "Page" %} {{ page_obj.number }} {% trans "of" %} {{ page_obj.paginator.num_pages }}
            </span>
            {% if page_obj.has_next %}
                <a href="?page={{ page_obj.next_page_number }}">{% trans "Next" %}</a>
                <a href="?page={{ page_obj.paginator.num_pages }}">{% trans "Last" %} &raquo;</a>
            {% endif %}
        </nav>
    {% endif %}
</div>
{% endblock %}

{% block extra_js %}
<script>
    document.querySelectorAll('.btn-primary').forEach(btn => {
        btn.addEventListener('click', function() {
            const productId = this.dataset.product;
            fetch(`/cart/add/${productId}/`, {
                method: 'POST',
                headers: {
                    'X-CSRFToken': '{{ csrf_token }}',
                    'Content-Type': 'application/json'
                }
            }).then(response => response.json())
              .then(data => console.log(data));
        });
    });
</script>
{% endblock %}
"#;

fn bench_format_simple(c: &mut Criterion) {
    c.bench_function("format_simple_template", |b| {
        let config = FormatterConfig::new(120, 4, None);
        let profile = Profile::Django;
        b.iter(|| {
            let _ = format_text(SIMPLE_TEMPLATE, &config, &profile);
        });
    });
}

fn bench_format_complex(c: &mut Criterion) {
    c.bench_function("format_complex_template", |b| {
        let config = FormatterConfig::new(120, 4, None);
        let profile = Profile::Django;
        b.iter(|| {
            let _ = format_text(COMPLEX_TEMPLATE, &config, &profile);
        });
    });
}

fn bench_format_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("format_by_size");
    let config = FormatterConfig::new(120, 4, None);
    let profile = Profile::Django;

    for size in [100, 500, 1000, 5000].iter() {
        let template = generate_template(*size);
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                let _ = format_text(&template, &config, &profile);
            });
        });
    }
    group.finish();
}

fn generate_template(lines: usize) -> String {
    let mut template = String::from("<!DOCTYPE html>\n<html>\n<body>\n");
    for i in 0..lines {
        template.push_str(&format!(
            "    <div class=\"item-{}\">\n        <p>{{{{ item_{} }}}}</p>\n    </div>\n",
            i, i
        ));
    }
    template.push_str("</body>\n</html>");
    template
}

criterion_group!(
    benches,
    bench_format_simple,
    bench_format_complex,
    bench_format_sizes
);
criterion_main!(benches);
