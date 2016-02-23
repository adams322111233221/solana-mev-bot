# perf4j - Performance Monitoring and Statistics for Java Code

  Perf4J is a set of utilities for calculating and displaying performance statistics for Java code. For developers who
  are familiar with logging frameworks such as log4j or logback, an analogy helps to describe Perf4J:

> _Perf4J is to System.currentTimeMillis() as log4j is to System.out.println()_

  How is this relevant to Perf4J? Consider that before good logging frameworks were widely available, developers new
  to Java would often print debugging statements using System.out.println(). Later they would discover they wanted
  log statements to go to a dedicated file, perhaps a log file that was rolled daily. Then they would discover they
  wanted finer control of which log statements would be written, and they wanted it to be possible to only have some
  log statements written in some environments without changing any code. Thus, the full set of features that
  log4j provides comes out of the original desire to have a "better" System.out.println() for logging statements.

  Similarly, when new Java developers discover that they need to time specified blocks of code for performance logging
  and monitoring reasons, they often do something like this:

```
long start = System.currentTimeMillis();
// execute the block of code to be timed
System.out.println("ms for block n was: " + (System.currentTimeMillis() - start));
```

  Later on, though, these developers find they want more information, such as aggregated performance statistics like
  mean, minimum, maximum, standard deviation and transactions per second over a set time span. They would like graphs
  of these values in real time to detect problems on a running server, or they would like to expose performance metrics
  through JMX. In addition, they want their timing statements to work with the common logging frameworks like log4j
  (and, more recently, SLF4J).

  Perf4J provides these features and more:

  * A simple stop watch mechanism for succinct timing statements.

  * A command line tool for parsing log files that generates aggregated statistics and performance graphs.

  * Easy integration with the most common logging frameworks and facades: log4j, java.util.logging, Apache Commons
    Logging and SLF4J (including logback).
    
  * Custom log4j and logback appenders to generate statistics and graphs in a running application.

  * The ability to expose performance statistics as JMX attributes, and to send notifications when statistics
    exceed specified thresholds.

  * A servlet for exposing performance graphs in a web application.

  * A `Profiled` annotation and a set of custom aspects that allow
    unobstrusive timing statements when coupled with an AOP framework such as AspectJ or Spring AOP.

  * An extensible architecture.

  See the Developer Guide to get started, or download Perf4J now.

# Integrated Tools

  Thanks to the folks that have integrated Perf4J into other frameworks:

  * [Grails Perf4j Integration Plugin](http://www.grails.org/plugin/perf4j), authored by Daniel Rinser

  * [Seam-Perf4j](http://seam-perf4j.sourceforge.net/), authored by Marcin Zaj&#261;czkowski

  * [Perf4CDI](http://perf4cdi.sourceforge.net/), authored by Marcin Zaj&#261;czkowski


# What's New

  Recently released version 0.9.16, the major new feature being custom logback appenders. Logback is now supported
  with the same level of functionality as log4j. Thanks to Xu Huisheng for the submission!
  
# What's Next

  Perf4J has been on hiatus for a while, but we are now starting back up with development, hoping to add more community
  involvement. To help with this effort, we've recently moved the code repository to GitHub,
  https://github.com/perf4j/perf4j, the main goal being to make it easier for contributors and maintainers to
  branch and merge code patches.

  If there are missing features you would like added, please let us know on our mailing lists,
  or better yet, download the source code and become a contributor.
