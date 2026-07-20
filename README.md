## Sommaire

- [Sommaire](#sommaire)
- [Présentation](#présentation)
- [Installation](#installation)
- [Accéder aux logs](#accéder-aux-logs)
- [Fichiers de configuration](#fichiers-de-configuration)
  - [Ajouter un groupe de paternes](#ajouter-un-groupe-de-paternes)
  - [Utilisation d'un modèle de reconnaissance pré entrainé](#utilisation-dun-modèle-de-reconnaissance-pré-entrainé)
- [Documentation développeur](#documentation-développeur)
- [Fonctionnement](#fonctionnement)
  - [Présentation des composants](#présentation-des-composants)
  - [AUI - Anonymizer User Interface](#aui---anonymizer-user-interface)
  - [AD2C - Anonymizer Data Control Center](#ad2c---anonymizer-data-control-center)
    - [Processus global](#processus-global)
    - [Traitement des documents](#traitement-des-documents)
  - [ARS - Anonymizer Recognition System](#ars---anonymizer-recognition-system)
  - [ADB - Anonymizer Database](#adb---anonymizer-database)
    - [Mappage des données](#mappage-des-données)
    - [Sécurisation](#sécurisation)
    - [Remarques](#remarques)

## Présentation

L'intelligence artificielle est aujourd'hui un incontournable permettant une amélioration concidérable de la productivité de chacun.

PromptGuard a donc été pensée pour permettre aux utilisateurs de profiter des différents outils utilisant l'intelligence artificielle tout en **réduisant les risques de fuites de données à caractères personnelles**.

## Installation

Le fichier `docker-compose.yaml` contient tous les paramètres nécessaire à une configuration minimale de l'application.

Pour importer l'application

```sh
git clone https://github.com/Enzo-CHT/promptguard.git
cd promptguard
```

Vous pouvez déployer l'application grâce au Makefile

```sh
make build
make up
```

> [!WARNING]
> L'accès à l'interface web peut prendre quelques minutes

## Accéder aux logs

Pour suivre les logs, vous pouvez utiliser la commande prévu par docker compose

```sh
make logs
```

Il est également possible d'**extraire les logs** du docker compose grâce à la commande :

```sh
docker compose -p promptguard cp  aui-svc:/mnt/logs/ /tmp/logs
```

## Fichiers de configuration

Les fichiers de configuration sont présents dans le dossier `src/config`. Dans celui-ci il existe deux fichiers de configuration sous le format **YAML**.

- `properties.yaml` - Pour configurer l'application en elle même.
- `recognizer.yaml` - Pour configurer le système de reconnaissance.

### Ajouter un groupe de paternes

Les **groupes de paternes** sont utilisés pour reconnaître des informations en se basant sur un ou plusieurs paterne(s) particulié(s).

Pour définir un groupe de paternes, il suffit de définir trois éléments :

- **Un nom de groupe** - qui indiquera l'information ciblée.
- **Un nom d'entité** - qui représente le type d'information et qui servira de masque.
- **Des paternes** - qui servirons a reconnaitre les éléments qui seront ajoutés dans le groupe.

Concernant ces paternes, on les définits grâce à trois éléments :

- **Un nom** - qui permet d'identifier quel paterne est utilisé
- **Un paterne** - grâce à une syntaxe regex
- **Un score** - qui permet de définir le niveau de confiance par rapport à la reconnaissance.

> [!WARNING]
> Si deux groupes différents reconnaissent le même éléments via leurs paternes, le score le plus élevé permettra de choisir à quelle groupe appartient l'élément.

```yaml
patterns_recognizers:
  # On définit un groupe appelé `phone_recognizer`
  - name: 'phone_recognizer'
    entity: 'PHONE'
    pattern:
      # On utilise différents paternes pour reconnaître le même élément
      - name: 'phone_pattern_1'
        regex: "\\b[0-9]{2} [0-9]{2} [0-9]{2}\\b"
        score: 0.5
      - name: 'phone_pattern_2'
        regex: "\\b[0-9]{2}-[0-9]{2}-[0-9]{2}\\b"
        score: 0.5
```

### Utilisation d'un modèle de reconnaissance pré entrainé

> Il est possible d'importer son propre modèle pour la reconnaissance d'information.

> [!NOTE]
> L'application intègre un modèle de base : [Babelscape\wikineural-multilingual-ner](https://huggingface.co/Babelscape/wikineural-multilingual-ner/tree/main)"

Pour ajouter un modèle, il suffit d'importer celui-ci dans le dossier `./models` puis de modifier le fichier de `recognizer.yaml`.

Vous devez par la suite préciser les entités reconnues dans la configuration.

```yaml
nlp_recognizers:
  nlp_engine_name: transformers
  models:
    - lang_code: fr
      model_name:
        spacy: fr_core_news_sm
        # Vous pouvez directement remplacer
        #`Babelscape/wikineural-multilingual-ner`
        # par le nom de votre modèle.
        transformers: /mnt/ml-models/Babelscape/wikineural-multilingual-ner
  # Precise les entités du modèle
  entities:
    - PERSON
    - DATE_TIME
    - EMAIL_ADDRESS
    - URL
    - AGE
    - LOCATION
```

> [!NOTE]
> La configuration YAML fonctionne de la même manière que celle intégrée dans la pipeline spaCy de Microsoft Presidio : [source](https://microsoft.github.io/presidio/analyzer/nlp_engines/transformers/)

Pour installer un modèle depuis Hugging face, vous pouvez utiliser la commande suivante en indiquant le nom du modèle.

```bash
huggingface-cli download <NOM_MODEL> --local-dir ./models
```

> [!WARNING]
> Assurez-vous que le modèle soit adapté avant de l'intégré dans la reconnaissance.
> Le modèle doit utiliser `Transformers` et être un modèle de `Token Classification`

## Documentation développeur

Pour obtenir une documentation technique des différents composants il suffit d'utiliser les commandes suivantes à partir de la racine du projet :

```sh
pip install --update pip
pip install pdoc3
pdoc --html --output-dir docs src/
```

Vous pouvez ensuite d'ouvrir le fichier `docs/src/index.html` pour avoir une vision de l'ensemble des modules qui compose le projet.

## Fonctionnement

### Présentation des composants

<img src="assets/global-diagram.png" alt="Diagram" width="250" height="auto" style="display: flex; margin: 40px auto;" >

promptguard est composée de 4 composants essentiels :

- **Guard User Interface (GUI)** - Gère l'interface web utilisateur
- **Data Control Center (D2C)** - Gère les requêtes utilisateur en fonction du type de requête. D2C agit comme un point d'inflexion entre les différents composants
- **Data Recognition System (DRS)** - Gère la reconnaissance de données à caractères personnelles ou autre informations sensibles
- **Guard Database** - Gère la désanonymisation des données

### AUI - Anonymizer User Interface

<img src="assets/aui-diagram.png" alt="Diagram" width="250" height="auto" style="display: flex; margin: 40px auto;" >

Comme expliqué plus haut l'GUI représente l'**interface utilisateur** et est le seul composant **normalement** accessible par les utilisateurs. Elle est composée d'un serveur `gunicorn` et d'une application web fonctionnant sous `Flask`.

Configuration du serveur Web

```sh
python3 -m gunicorn "gui_server:create_app()" \
-b 0.0.0.0:8000 \
-u gui \
-g gui \
--workers 5 \
--threads 6 \
--preload \
--timeout 90 \
--log-level info \
--worker-tmp-dir /dev/shm \
```

Lorsque l'utilisateur intéragit avec l'interface web, le controlleur réceptionne les requêtes.

Le _Controller_ traduit ensuite ces requêtes web en une instance de `UserRequest`. Cette instance est ensuite transmise au _AUICore_ qui se charge d'établir une connexion avec le serveur **D2C** (via le client **D2C**). Puis il utilise les informations contenu dans l'instance de `UserRequest` pour envoyer une requête interprétable par l'D2C.

### D2C - Anonymizer Data Control Center

<img src="assets/D2C-diagram.png" alt="Diagram" width="250" height="auto" style="display: flex; margin: 40px auto;" >

Le rôle de l'D2C est, comme indiqué précédemment, d'agir comme carrefour de l'application.

Il est à la fois celui qui communique avec tous les composants mais également celui qui gère la transformation des données.

#### Processus global

Lorsqu'un utilisateur souhaite anonymiser un type de donnée, D2C se charge dans un premier temps de **transformer la donnée source en une donnée interprétable** par les autres composants.

> [!NOTE] Exemple
>
> Le système de reconnaissance base sa reconnaissance sur du texte pour mener à bien son rôle. L'objectif d'D2C est donc d'adapter la donnée pour le système de reconnaissance, puis, après traitement, reconvertir la donnée en son état initial en incluant les modifications d'anonymisation.

---

> En résumé
>
> 1. D2C reçoit des données de l'utilisateur
> 2. Il adapte ces données pour qu'elles puissent être comprises par les autres composants
> 3. Il envoie les données aux différents composants
> 4. Il centralise les résultats
> 5. D2C reconstitue la donnée initiale en incluant les modification d'anonymisation
> 6. Finalement, il envoie une réponse à l'AUI qui se charge d'afficher le résultat à l'utilisateur
>
> <img src="assets/d2c-process-diagram.png" alt="Diagram" width="500" height="auto" style="display: flex; margin: 40px auto;" >

#### Traitement des documents

Comme on la vue, D2C doit **adapté les documents** pour que les autres composants puissent entreprendre leur traitement.

Lorsqu'il s'agit déjà de texte, il n'y a rien à faire.
En revenche,dès lors qu'il s'agit d'un autre type de donnée, il est nécessaire de pouvoir convertir celle-ci facilement tout en garantissant un retour arrière.

Pour se faire, Il a donc été décidé d'utiliser l'océrisation sur image. De cette manière, il suffit de convertir un document en image pour extraire le texte. Appliquer les modifications à l'image avant de retransformer celle-ci au format d'origine.

**Avantages**:

1. Facile
2. Compatible à la majorité des documents
3. Aucun retour en arrière possible par l'IA

<br>

> En résumé
>
> 1. l'AD2C récupère un document.
> 2. Il convertie ce document en image
> 3. Il extrait le texte de l'image par océrisation
> 4. Il distribue le texte aux différents composants
> 5. Après avoir récupéré les résultats, il applique les modifications sur l'image
> 6. Et finalement, il convertie l'image au format initial

<br>

> [!NOTE]
> Il s'agit là de la meilleure solution trouvée jusqu'à présent pour extraire du texte d'un document et appliquer des modifications sans réduire la qualité du document.

### ARS - Anonymizer Recognition System

<img src="assets/ars-diagram.png" alt="Diagram" width="250" height="auto" style="display: flex; margin: 40px auto;" >

Le rôle de l'ARS est de reconnaître les informations catégorisées comme "sensibles" en se basant sur différents mécanismes de reconnaissance.

> // en cours de rédaction

### ADB - Guard Database

<img src="assets/gdb-diagram.png" alt="Diagram" width="250" height="auto" style="display: flex; margin: 40px auto;" >

L'GDB est le composant qui **permet la désanonymisation des données**. Lorsque l'D2C applique l'anonymisation, celui-ci communique avec l'GDB.

L'information à l'origine est enregistrée dans une base de données **PostgreSql** et GDB renvoie un "masque" qui la remplacera.

> [!WARNING]
> La désanonymisation ne concerne que le traitement de texte. Les documents ne sont pas concernés

#### Mappage des données

Dans `src/config/properties.yaml` il est possible d'activer ou non le mappage des données :

```yaml
# Propriétés global de l'application
properties:
  debug: False
  mapping_table: True # ici
  verbose: False
```

Si cette option est activée, le masque intégrera un identifiant unique qui sera utilisé pour permettre à l'utilisateur d'inverser le processus d'anonymisation.

- Exemple masque : `{{PERSON}}`
- Exemple masque + identifiant : `{{PERSON 4d4f8}}`

#### Sécurisation

Pour faire le lien entre une information anonymisée et un utilisateur, promptguard se base sur **l'identifiant de mappage de l'utilisateur**. Celui-ci est généré par l'GUI lors de l'intéraction de l'utilisateur avec l'application. Puis, il est stocker dans les cookies du navigateur.

Au niveau de la base de données, chaque information est chiffrée via un **chiffrement AES** afin qu'elles ne puissent être récupérées.

#### Remarques

- La base de données ne peut ajouter un attribut si l'élément à anonymiser est de taille supérieur à 120 caractères.
- Si la base de données ne peut enregistrer un élément, alors le masque de celui-ci ne sera pas associé à un identifiant unique.
