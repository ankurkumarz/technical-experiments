# dbt (Data Build Tool)

## Context

- [dbt](https://www.getdbt.com/) is a SQL-first transformation workflow that lets teams quickly and collaboratively deploy analytics code following software engineering best practices like modularity, portability, CI/CD, and documentation.
- dbt uses Jinja as the templating engine to generate SQL code for data transformations such as  {{ ref() }} function:

```
select *
from {{ref('model_a')}}
```

- [Jinja Tutorial](https://docs.getdbt.com/guides/using-jinja?step=1)

## dbt Setup in Cloud

- [dbt Cloud](https://auth.cloud.getdbt.com/login) is a hosted service for managing your dbt projects and deployments launched in 2019. [Click here](https://www.getdbt.com/blog/introducing-dbt-cloud) to know more details.
- [dbt Reference](https://docs.getdbt.com/reference/references-overview)
- Login credentials: <hotmail id>@hotmail.com / <passwd>

## dbt Setup on Local

- Install dbt project [Click here](https://docs.getdbt.com/docs/core/installation-overview) to know more:

```
brew update
brew tap dbt-labs/dbt
brew install dbt-postgres
conda create -e dbt
pip install dbt-postgres
```

- Create a dbt project:

```
dbt init dbt_sandbox
dbt build
dbt run
```

- Different between dbt build & run:
dbt run = execute your models. dbt build = execute your models and test them.

## dbt - Getting started Guides

- [dbt Guides](https://docs.getdbt.com/guides)
- [dbt Fundamentals Course](https://courses.getdbt.com/courses/fundamentals)
- [dbt with BiqQuery](https://docs.getdbt.com/guides/bigquery?step=1)

## Jinja Examples

- [Complete Examples](https://docs.getdbt.com/guides/using-jinja?step=10)

- For loop:
```
{% set payment_methods = ["bank_transfer", "credit_card", "gift_card"] %}

select
order_id,
{% for payment_method in payment_methods %}
sum(case when payment_method = '{{payment_method}}' then amount end) as {{payment_method}}_amount,
{% endfor %}
sum(amount) as total_amount
from {{ ref('raw_payments') }}
group by 1
```

- Writing modular macros:

```
{% macro get_column_values(column_name, relation) %}

{% set relation_query %}
select distinct
{{ column_name }}
from {{ relation }}
order by 1
{% endset %}

{% set results = run_query(relation_query) %}

{% if execute %}
{# Return the first column #}
{% set results_list = results.columns[0].values() %}
{% else %}
{% set results_list = [] %}
{% endif %}

{{ return(results_list) }}

{% endmacro %}


{% macro get_payment_methods() %}

{{ return(get_column_values('payment_method', ref('raw_payments'))) }}

{% endmacro %}
```